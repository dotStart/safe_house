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
use crate::cfg::ApplicationConfig;
use crate::feature::system::InternalUserSetup;
use crate::feature::FeatureState;
use crate::routes::app::response;
use crate::security::auth::AuthenticationManager;
use rocket::serde::json::Json;
use rocket::State;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_COMMIT_HASH: &'static str = env!("GIT_COMMIT_ID");
const APP_REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

/// Exposes basic application build metadata as well as configuration information necessary for
/// use with the web UI and other clients.
///
/// # Example Request
///
/// ```http
/// GET /v1 HTTP/1.1
/// Accept: application/json
/// ```
///
/// # Example Response
///
/// ```http
/// HTTP/1.1 200 Ok
/// Content-Type: application/json
///
/// {
///     "name": "safe_house",
///     "version": "0.1.0",
///     "repository": "https://github.com/dotStart/safe_house",
///     "config": {
///         "security": {
///             "auth": "Internal",
///             "key_derivation_rounds": 60000
///         }
///     }
/// }
/// ```
#[get("/")]
pub fn root(
    setup: &FeatureState<InternalUserSetup>,
    config: &State<ApplicationConfig>,
    auth: &State<AuthenticationManager>,
) -> Json<response::AppInfo> {
    let mut version = APP_VERSION.to_string();
    if APP_COMMIT_HASH.len() != 0 {
        version += format!("+git-{}", APP_COMMIT_HASH).as_str();
    }

    Json(response::AppInfo {
        name: APP_NAME,
        version,
        repository: APP_REPOSITORY,
        config: response::Config {
            setup: setup.is_available(),
            security: response::Security {
                auth: auth.system(),
                key_derivation_rounds: config.document.key_derivation_rounds,
            },
        },
    })
}
