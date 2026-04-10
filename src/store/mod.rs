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
pub mod db;
pub mod document;
pub mod error;
pub mod internal_user;
pub mod storage;
pub mod system;
pub mod types;

use rocksdb::{DBCompactionStyle, DBCompressionType, TransactionDB};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;

pub fn open_database(cfg: &Config) -> Result<Arc<TransactionDB>, DatabaseInitError> {
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    opts.set_compression_type(if cfg.compress {
        DBCompressionType::Zlib
    } else {
        DBCompressionType::None
    });
    opts.create_missing_column_families(true);
    opts.set_enable_blob_files(true);
    opts.set_min_blob_size(10_000_000); // TODO: Configurable
    opts.set_compaction_style(DBCompactionStyle::Level); // recommended when blob storage is enabled

    let tx_opts = rocksdb::TransactionDBOptions::default();
    let cfs = vec![
        document::FAMILY_NAME.to_string(),
        internal_user::FAMILY_NAME.to_string(),
        system::FAMILY_NAME.to_string(),
    ];

    match TransactionDB::open_cf(&opts, &tx_opts, cfg.path.as_path(), cfs) {
        Ok(db) => Ok(Arc::new(db)),
        Err(e) => Err(DatabaseInitError(e.to_string())),
    }
}

pub struct Config {
    pub path: PathBuf,
    pub compress: bool,
}

pub struct DatabaseInitError(String);

impl Display for DatabaseInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
