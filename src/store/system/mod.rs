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
pub mod error;

use crate::store::db::RawStore;
use crate::store::error::StoreError;
use crate::store::system::error::SystemStoreError;
use byteorder::{BigEndian, ByteOrder};
use rand::{thread_rng, RngCore};
use rocksdb::TransactionDB;
use std::sync::Arc;

pub const FAMILY_NAME: &'static str = "safe_house";

const SCHEMA_VERSION_KEY: &'static str = "schema_version";
const JWT_SECRET_KEY: &'static str = "jwt_secret";

const SCHEMA_VERSION: u64 = 0;

pub struct Repository {
    store: RawStore,
}

impl Repository {
    pub fn new(db: &Arc<TransactionDB>) -> Result<Self, SystemStoreError> {
        let store = RawStore::new(db, FAMILY_NAME);
        let repository = Self { store };

        if let Some(version) = repository
            .schema_version()
            .map_err(|e| SystemStoreError::GenericStoreError(e))?
        {
            if version != SCHEMA_VERSION {
                return Err(SystemStoreError::UnknownSchemaVersion(version));
            }
        } else {
            repository
                .set_schema_version(SCHEMA_VERSION)
                .map_err(|e| SystemStoreError::GenericStoreError(e))?;
        }

        Ok(repository)
    }

    pub fn schema_version(&self) -> Result<Option<u64>, StoreError> {
        Ok(self
            .store
            .load(SCHEMA_VERSION_KEY)?
            .map(|encoded| BigEndian::read_u64(encoded.as_ref())))
    }

    fn set_schema_version(&self, version: u64) -> Result<(), StoreError> {
        let mut encoded = [0; 8];
        BigEndian::write_u64(encoded.as_mut(), version);

        self.store.store(SCHEMA_VERSION_KEY, &Vec::from(encoded))?;
        Ok(())
    }

    pub fn jwt_secret(&self) -> Result<Vec<u8>, StoreError> {
        let tx = self.store.transaction()?;

        let stored = tx.load(JWT_SECRET_KEY)?;
        if let Some(key) = stored {
            return Ok(key);
        }

        let mut key = Vec::<u8>::with_capacity(32);
        let mut rng = thread_rng();
        rng.fill_bytes(key.as_mut());

        tx.store(JWT_SECRET_KEY, &key)?;
        tx.commit()?;

        Ok(key)
    }

    pub fn generate_jwt_secret(&self) -> Result<Vec<u8>, StoreError> {
        let mut key = Vec::<u8>::with_capacity(32);
        let mut rng = thread_rng();
        rng.fill_bytes(key.as_mut());

        self.store.store(JWT_SECRET_KEY, &key)?;

        Ok(key)
    }
}
