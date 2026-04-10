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
use crate::feature::auth::InternalAuth;
use crate::feature::Feature;
use crate::routes::auth::error::LoginError;
use crate::routes::auth::request::LoginParameters;
use crate::security::auth::{AuthenticationManager, User};
use crate::store::internal_user::entity::Username;
use rocket::serde::json::Json;
use rocket::State;

#[post("/login", data = "<params>")]
pub async fn login(
    _f: &Feature<InternalAuth>,
    auth: &State<AuthenticationManager>,
    params: Json<LoginParameters<'_>>,
) -> Result<Json<String>, LoginError> {
    if !params.username.is_valid() {
        return Err(LoginError::InvalidUsername(
            "Name contains invalid characters",
        ));
    }

    Ok(Json(
        auth.login(&params.username, &params.password)
            .await
            .map_err(|e| LoginError::AuthenticationFailure(e.to_string()))?
            .to_string(),
    ))
}

#[post("/token")]
pub async fn refresh(
    _f: &Feature<InternalAuth>,
    auth: &State<AuthenticationManager>,
    user: User<'_>,
) -> Result<Json<String>, LoginError> {
    Ok(Json(
        auth.refresh_token(&Username::new(user.id().to_string()))
            .await
            .map_err(|e| LoginError::AuthenticationFailure(e.to_string()))?
            .to_string(),
    ))
}
