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
use config::{Map, Value, ValueKind};
use rocket::serde::Deserialize;
use rocket::Data;

#[derive(Clone, Deserialize, Debug)]
pub struct Database {
    pub path: String,
    pub compress: bool,
}

impl Database {
    pub fn dump_to_log(&self) {
        info!("   >> db.path: {}", self.path);
        info!("   >> db.comress: {}", self.compress);
    }

    pub fn collect(&self) -> Value {
        Value::new(
            None,
            ValueKind::Table(Map::from([
                (
                    "path".to_string(),
                    Value::new(None, ValueKind::String(self.path.clone())),
                ),
                (
                    "compress".to_string(),
                    Value::new(None, ValueKind::Boolean(self.compress)),
                ),
            ])),
        )
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: "./data".to_string(),
            compress: true,
        }
    }
}
