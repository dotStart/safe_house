/*
 * This file is part of safe_house - a simple E2E encrypted file sharing utility.
 * Copyright (c) 2026 Yuki Donath <https://dotstart.tv> and other contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
pub mod entity;

use crate::security::permission::PermissionFlag;
use crate::store::db::{Store, StoreOps};
use crate::store::error::StoreError;
use crate::store::internal_user::entity::{Parameters, Username};
use crate::store::system::migration::Migrate;
use crate::store::system::SchemaVersion;
use crate::{deny_latest_schema, panic_latest_schema};
use rocksdb::{IteratorMode, TransactionDB};
use std::sync::Arc;

pub const FAMILY_NAME: &'static str = "internal_user";

pub struct Repository {
    store: Store<Parameters>,
}

impl Repository {
    pub fn new(db: &Arc<TransactionDB>) -> Self {
        Repository {
            store: Store::new(db, FAMILY_NAME),
        }
    }

    pub fn create(&self, name: &Username, parameters: &Parameters) -> Result<(), StoreError> {
        if !name.is_valid() {
            return Err(StoreError::ValidationFailure(
                "malformed username".to_string(),
            ));
        }

        self.store.store(name.as_str(), parameters)?;
        Ok(())
    }

    pub fn count(&self) -> Result<usize, StoreError> {
        Ok(self.store.iterator(IteratorMode::Start)?.count())
    }

    pub fn list(&self) -> Result<Vec<(Username<'_>, Box<Parameters>)>, StoreError> {
        let it = self.store.iterator(IteratorMode::Start)?;
        let mut results = Vec::new();

        for e in it {
            match e {
                Ok(kv) => results.push((Username::new(kv.0.to_string()), kv.1)),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    pub fn get(&self, name: &Username) -> Result<Option<Parameters>, StoreError> {
        self.store.load(name.as_str())
    }

    pub fn update<O>(&self, name: &Username, operation: O) -> Result<Option<Parameters>, StoreError>
    where
        O: FnOnce(&mut Parameters),
    {
        let tx = self.store.transaction()?;
        let mut parameters = match tx.load(name.as_str())? {
            Some(stored) => stored,
            None => return Ok(None),
        };

        operation(&mut parameters);
        tx.store(name.as_str(), &parameters)?;

        tx.commit()?;
        Ok(Some(parameters))
    }

    pub fn delete(&self, name: &Username) -> Result<(), StoreError> {
        self.store.delete(name.as_str())
    }
}

impl Migrate for Repository {
    fn migrate(&self, from_version: SchemaVersion) -> Result<(), StoreError> {
        deny_latest_schema!(from_version);

        match from_version {
            SchemaVersion::Initial => {
                let tx = self.store.transaction()?;
                let it = tx.iterator(IteratorMode::Start)?;

                for e in it {
                    match e {
                        Ok((key, mut params)) => {
                            if params.has_any_permission(PermissionFlag::All) {
                                params.permissions |= PermissionFlag::BypassRateLimit;
                                tx.store(key.as_str(), params.as_ref())?;
                            }
                        }
                        _ => {}
                    }
                }

                tx.commit()?;
            }
            SchemaVersion::V1 => panic_latest_schema!(),
        }

        Ok(())
    }
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}
