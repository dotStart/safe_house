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
use crate::security::auth::AuthenticationSystem;
use rocket::serde::Serialize;

/// Encapsulates basic application metadata and configuration information for clients.
///
/// # Example Body
///
/// ```json
/// {
///     "name": "safe_house",
///     "version": "0.1.0",
///     "repository": "https://github.com/dotStart/safe_house",
///     "config": { ... }
/// }
/// ```
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppInfo {
    pub name: &'static str,
    pub version: String,
    pub repository: &'static str,
    pub config: Config,
}

/// Encapsulates basic configuration information for clients.
///
/// # Example Body
///
/// ```json
/// {
///     "security": { ... }
/// }
/// ```
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub setup: bool,
    pub security: Security,
}

/// Encapsulates security relevant information for clients.
///
/// # Example Body
///
/// ```json
/// {
///     "auth": "Internal",
///     "key_derivation_rounds": 60000
/// }
/// ```
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Security {
    pub auth: AuthenticationSystem,
    pub key_derivation_rounds: u64,
}
