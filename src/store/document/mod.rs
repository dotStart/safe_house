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
use crate::store::db::{Store, StoreOps};
use crate::store::document::entity::Parameters;
use crate::store::error::StoreError;
use crate::store::types::Clover;
use chrono::Utc;
use rocksdb::{IteratorMode, TransactionDB};
use std::str::FromStr;
use std::sync::Arc;

pub mod entity;
pub mod cleanup;

pub const FAMILY_NAME: &'static str = "document";

pub struct Repository {
    store: Store<Parameters>,
}

impl Repository {
    pub fn new(db: &Arc<TransactionDB>) -> Self {
        Repository {
            store: Store::new(db, FAMILY_NAME),
        }
    }

    pub fn create(&self, parameters: &Parameters) -> Result<Clover<'_>, StoreError> {
        let clover = Clover::generate();

        self.store.store(clover.as_str(), parameters)?;
        Ok(clover)
    }

    pub fn get(&self, clover: &Clover) -> Result<Option<Parameters>, StoreError> {
        let tx = self.store.transaction()?;

        let params = match tx.load(clover.as_str())? {
            Some(p) => p,
            None => return Ok(None),
        };

        // ensure that invalid documents are deleted now so they don't linger and longer than
        // necessary
        if !params.is_valid(Utc::now()) {
            tx.delete(clover.as_str())?;
            tx.commit()?;
            Ok(None)
        } else {
            tx.rollback()?;
            Ok(Some(params))
        }
    }

    pub fn list(&self) -> Result<Vec<(Clover<'_>, Box<Parameters>)>, StoreError> {
        let now = Utc::now();

        let tx = self.store.transaction()?;
        let it = tx.iterator(IteratorMode::Start)?;
        let mut result = Vec::new();

        for e in it {
            match e {
                Ok((k, v)) => {
                    if v.is_valid(now) {
                        let clover = Clover::from_str(k.as_str()).unwrap();
                        result.push((clover, v));
                    } else {
                        tx.delete(k.as_str())?;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }

    pub fn expire(&self) -> Result<usize, StoreError> {
        let now = Utc::now();

        let tx = self.store.transaction()?;
        let it = tx.iterator(IteratorMode::Start)?;

        let mut expired_keys = Vec::<String>::new();
        for e in it {
            match e {
                Ok((k, v)) => {
                    if !v.is_valid(now) {
                        expired_keys.push(k.to_string())
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to check expiration for one or more documents: {}",
                        e
                    );
                }
            }
        }

        for key in &expired_keys {
            if let Err(e) = tx.delete(key.as_str()) {
                warn!("Failed to delete expired document \"{}\": {}", key, e);
            }
        }

        tx.commit()?;
        Ok(expired_keys.len())
    }

    pub fn update<O>(&self, clover: &Clover, operation: O) -> Result<Option<Parameters>, StoreError>
    where
        O: FnOnce(&mut Parameters),
    {
        let tx = self.store.transaction()?;
        let mut parameters = match tx.load(clover.as_str())? {
            Some(stored) => stored,
            None => return Ok(None),
        };

        // prevent already expired documents from being updated as these aren't supposed to be here
        // anymore in the first place
        let request_time = Utc::now();
        if !parameters.is_valid(request_time) {
            tx.delete(clover.as_str())?;
            tx.commit()?;
            return Ok(None);
        }

        operation(&mut parameters);

        // documents are a little special since we support per-view expiration where documents are
        // deleted once viewed n times, as such, we take this extra validation step here to remove
        // invalid documents
        if parameters.is_valid(request_time) {
            tx.store(clover.as_str(), &parameters)?;
        } else {
            tx.delete(clover.as_str())?;
        }

        tx.commit()?;
        Ok(Some(parameters))
    }

    pub fn delete(&self, clover: &Clover) -> Result<(), StoreError> {
        self.store.delete(clover.as_str())
    }

    pub fn delete_own(&self, clover: &Clover, expected_owner_id: &str) -> Result<bool, StoreError> {
        let tx = self.store.transaction()?;

        let document = match tx.load(clover.as_str())? {
            Some(params) => params,
            None => return Ok(false),
        };

        if !document.owner_id.eq(expected_owner_id) {
            return Ok(false);
        }

        tx.delete(clover.as_str())?;
        tx.commit()?;

        Ok(true)
    }
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}
