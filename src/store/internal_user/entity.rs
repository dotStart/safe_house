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
use crate::security::auth::ANONYMOUS_ID;
use crate::security::hash::HashProvider;
use crate::security::permission::PermissionFlag;
use crate::store::error::StoreError;
use crate::store::storage::Document;
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use zxcvbn::{zxcvbn, Score};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Username<'a>(Cow<'a, str>);

impl Username<'_> {
    pub fn new(content: String) -> Self {
        Username(Cow::Owned(content))
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() >= 3
            && self.0.chars().all(|c| c.is_ascii_alphanumeric())
            && !ANONYMOUS_ID.eq(self.as_str())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Username<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Username<'_> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Username(s.to_owned().into()))
    }
}

impl<'a> FromParam<'a> for Username<'a> {
    type Error = std::convert::Infallible;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Username(param.to_owned().into()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub display_name: Option<String>,
    pub password_hash: String,
    pub permissions: PermissionFlag,
}

impl Parameters {
    pub fn validate_password(password: &str) -> bool {
        zxcvbn(password, &[]).score() >= Score::Three
    }

    pub fn set_password(&mut self, password: &str) -> Result<(), StoreError> {
        self.password_hash = HashProvider::hash(password)
            .map_err(|e| StoreError::UpdateFailure(format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn check_password(&self, password: &str) -> bool {
        HashProvider::check(&self.password_hash, password).unwrap_or_else(|e| {
            error!("Password check failed: {}", e);
            false
        })
    }

    pub fn has_any_permission(&self, permission: PermissionFlag) -> bool {
        self.permissions.contains_any(&permission)
    }
}

impl Document for Parameters {
    fn load(serialized: &Vec<u8>) -> Result<Self, StoreError> {
        rmp_serde::from_slice(serialized.as_slice())
            .map_err(|e| StoreError::DeserializationFailure(e.to_string()))
    }

    fn store(&self) -> Result<Vec<u8>, StoreError> {
        rmp_serde::to_vec(self).map_err(|e| StoreError::SerializationFailure(e.to_string()))
    }
}
