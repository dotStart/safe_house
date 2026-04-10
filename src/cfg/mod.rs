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
pub mod security;

use crate::cfg::db::Database;
use crate::cfg::document::Document;
use crate::cfg::security::Security;
use config::{ConfigError, Map, Source, Value};
use rocket::serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct ApplicationConfig {
    pub database: Database,
    pub security: Security,
    pub document: Document,
}

impl ApplicationConfig {
    pub const LOCATION: &'static str = "safe_house.toml";

    pub fn defaults() -> Self {
        Self {
            database: Database::defaults(),
            security: Security::defaults(),
            document: Document::defaults(),
        }
    }

    pub fn dump_to_log(&self) {
        info!("safe_house Configuration:");

        self.database.dump_to_log();
        self.security.dump_to_log();
        self.document.dump_to_log();
    }
}

impl Source for ApplicationConfig {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        // TODO: Custom macro?
        Ok(Map::from([
            ("database".to_string(), self.database.collect()),
            ("security".to_string(), self.security.collect()),
            ("document".to_string(), self.document.collect()),
        ]))
    }
}
