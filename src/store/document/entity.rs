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
use crate::store::error::StoreError;
use crate::store::storage::Document;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub remaining_views: Option<u16>,
    pub expires_at: Option<DateTime<Utc>>,
    pub key_derivation_rounds: u64,
    pub iv: Vec<u8>,
    pub salt: Vec<u8>,
    pub content: Vec<u8>,
}

impl Document for Parameters {
    fn load(serialized: &Vec<u8>) -> Result<Parameters, StoreError> {
        rmp_serde::from_slice(serialized.as_slice())
            .map_err(|e| StoreError::DeserializationFailure(e.to_string()))
    }

    fn store(&self) -> Result<Vec<u8>, StoreError> {
        rmp_serde::to_vec(self).map_err(|e| StoreError::SerializationFailure(e.to_string()))
    }
}

impl Parameters {
    pub fn record_view(&mut self) {
        match self.remaining_views {
            Some(remaining) => self.remaining_views = Some(remaining - 1),
            None => {}
        }
    }

    pub fn has_expired(&self, at: DateTime<Utc>) -> bool {
        match self.expires_at {
            Some(expires_at) => at > expires_at,
            None => false,
        }
    }

    pub fn has_remaining_views(&self) -> bool {
        match self.remaining_views {
            Some(remaining) => remaining != 0,
            None => true,
        }
    }

    pub fn is_valid(&self, at: DateTime<Utc>) -> bool {
        !self.has_expired(at) && self.has_remaining_views()
    }
}
