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
use crate::feature::system::InternalUserSetup;
use crate::feature::Feature;
use crate::routes::setup::error::UserSetupError;
use crate::routes::setup::request::UserParameters;
use crate::security::hash::HashProvider;
use crate::security::permission::PermissionFlag;
use crate::store::internal_user::entity::Parameters;
use crate::store::internal_user::Repository;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use zxcvbn::{zxcvbn, Score};

#[post("/user", data = "<params>")]
pub fn initial_user(
    _f: &Feature<InternalUserSetup>,
    store: &State<Repository>,
    params: Json<UserParameters>,
) -> Result<Status, UserSetupError> {
    if !params.name.is_valid() {
        warn!("Invalid username specified during setup");
        return Err(UserSetupError::InvalidUsername("Invalid username format"));
    }

    let password_score = zxcvbn(params.password.as_str(), &[]);
    if password_score.score() < Score::Three {
        return Err(UserSetupError::InsufficientPasswordStrength(format!(
            "Expected password with score of 3 but got {}",
            password_score.score()
        )));
    }

    let password_hash = HashProvider::hash(params.password.as_str()).map_err(|e| {
        error!("Password hash failure during setup: {}", e);
        UserSetupError::PasswordHashFailure("Password hash failure")
    })?;

    match store.create(
        &params.name,
        &Parameters {
            display_name: params.display_name.clone(),
            password_hash,
            is_password_expired: false,
            permissions: PermissionFlag::All,
        },
    ) {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to write setup user to database: {}", e);
            Err(UserSetupError::StoreFailure("Failed to write to database"))
        }
    }
}
