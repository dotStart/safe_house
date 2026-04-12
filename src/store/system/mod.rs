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
pub mod migration;

use std::fmt::{Display, Formatter};
use crate::store::db::RawStore;
use crate::store::error::StoreError;
use byteorder::{BigEndian, ByteOrder};
use rand::rngs::ThreadRng;
use rand::Rng;
use rocksdb::TransactionDB;
use std::sync::Arc;

pub const FAMILY_NAME: &'static str = "safe_house";

const SCHEMA_VERSION_KEY: &'static str = "schema_version";
const JWT_SECRET_KEY: &'static str = "jwt_secret";

#[derive(Clone)]
pub struct Repository {
    store: RawStore,
}

impl Repository {
    pub fn new(db: &Arc<TransactionDB>) -> Self {
        let store = RawStore::new(db, FAMILY_NAME);
        Self { store }
    }

    pub fn check_migration(&self) -> Result<MigrationResult, StoreError> {
        Ok(match self.schema_version()? {
            Some(version) => {
                if version == SchemaVersion::LATEST {
                    MigrationResult::UpToDate(version)
                } else if version < SchemaVersion::LATEST {
                    MigrationResult::Required(version)
                } else {
                    MigrationResult::UnknownVersion(version)
                }
            }
            None => {
                self.complete_migration()?;
                MigrationResult::UpToDate(SchemaVersion::LATEST)
            }
        })
    }

    pub fn complete_migration(&self) -> Result<SchemaVersion, StoreError> {
        self.set_schema_version(SchemaVersion::LATEST)?;
        Ok(SchemaVersion::LATEST)
    }

    pub fn schema_version(&self) -> Result<Option<SchemaVersion>, StoreError> {
        Ok(self
            .store
            .load(SCHEMA_VERSION_KEY)?
            .and_then(|encoded| SchemaVersion::from_u64(BigEndian::read_u64(encoded.as_ref()))))
    }

    fn set_schema_version(&self, version: SchemaVersion) -> Result<(), StoreError> {
        let mut encoded = [0; 8];
        BigEndian::write_u64(encoded.as_mut(), version.to_u64());

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
        let mut rng = ThreadRng::default();
        rng.fill_bytes(key.as_mut());

        tx.store(JWT_SECRET_KEY, &key)?;
        tx.commit()?;

        Ok(key)
    }

    pub fn generate_jwt_secret(&self) -> Result<Vec<u8>, StoreError> {
        let mut key = Vec::<u8>::with_capacity(32);
        let mut rng = ThreadRng::default();
        rng.fill_bytes(key.as_mut());

        self.store.store(JWT_SECRET_KEY, &key)?;

        Ok(key)
    }
}

macro_rules! schema_version {
    ( @latest $head:ident, ) => {
        pub const LATEST: SchemaVersion = Self::$head;
    };
    ( @latest $_head:ident, $($tail:ident,)* ) => {
        schema_version!(@latest $($tail,)*);
    };
    ( $($name:ident = $value:expr),* ) => {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
        pub enum SchemaVersion {
            $(
                $name,
            )*
        }

        impl SchemaVersion {
            schema_version!(@latest $($name,)*);

            pub const fn from_u64(value: u64) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Self::$name),
                    )*
                    _ => None,
                }
            }

            pub const fn to_u64(&self) -> u64 {
                match self {
                    $(
                        Self::$name => $value,
                    )*
                }
            }

            pub fn is_latest(&self) -> bool {
                *self == Self::LATEST
            }
        }

        impl Display for SchemaVersion {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$name => f.write_str($value.to_string().as_str()),
                    )*
                }
            }
        }
    }
}

schema_version!(Initial = 0, V1 = 1);

pub enum MigrationResult {
    /// Indicates that store migration is required before the application can be used.
    Required(SchemaVersion),

    /// Indicates that the schema version is unknown and that the application should thus shut down
    /// before damaging the store.
    UnknownVersion(SchemaVersion),

    /// Indicates that the store is up to date.
    UpToDate(SchemaVersion),
}

#[macro_export]
macro_rules! panic_latest_schema {
    () => {
        panic!("Migration from latest is unsupported")
    };
}

#[macro_export]
macro_rules! deny_latest_schema {
    ($name:ident) => {
        if $name.is_latest() {
            panic_latest_schema!()
        }
    };
}