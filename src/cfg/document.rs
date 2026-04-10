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

#[derive(Clone, Deserialize, Debug)]
pub struct Document {
    pub expiration_job_minutes: u64,
    pub key_derivation_rounds: u64,
}

impl Document {
    pub fn defaults() -> Self {
        Self {
            expiration_job_minutes: 30,
            key_derivation_rounds: 600_000,
        }
    }

    pub fn dump_to_log(&self) {
        info!(
            "   >> document.expiration_job_minutes: {}",
            self.expiration_job_minutes
        );
        info!(
            "   >>> document.key_derivation_rounds: {}",
            self.key_derivation_rounds
        );
    }

    pub fn collect(&self) -> Value {
        Value::new(
            None,
            ValueKind::Table(Map::from([
                (
                    "expiration_job_minutes".to_string(),
                    Value::new(None, ValueKind::U64(self.expiration_job_minutes)),
                ),
                (
                    "key_derivation_rounds".to_string(),
                    Value::new(None, ValueKind::U64(self.key_derivation_rounds)),
                ),
            ])),
        )
    }
}
