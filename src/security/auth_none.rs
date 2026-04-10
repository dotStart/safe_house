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
use crate::security::auth;
use crate::security::auth::{AuthenticationSystem, LoginError, User, ANONYMOUS_DISPLAY_NAME};
use crate::security::permission::PermissionFlag;
use crate::security::token::Token;
use crate::store::internal_user::entity::Username;
use rocket::fairing::AdHoc;
use std::sync::Arc;

pub fn stage() -> AdHoc {
    auth::stage(AuthenticationProvider::new())
}

pub struct AuthenticationProvider;

impl AuthenticationProvider {
    pub fn new() -> Arc<Self> {
        Arc::new(AuthenticationProvider)
    }
}

#[rocket::async_trait]
impl auth::AuthenticationProvider for AuthenticationProvider {
    fn system(&self) -> AuthenticationSystem {
        AuthenticationSystem::None
    }

    fn anonymous(&self) -> User<'_> {
        User::anonymous(PermissionFlag::All)
    }

    async fn display_name(&self, _user_id: &str) -> Option<String> {
        Some(ANONYMOUS_DISPLAY_NAME.to_string())
    }

    async fn login(&self, _name: &Username, _password: &str) -> Result<Token, LoginError> {
        Err(LoginError::UnsupportedOperation)
    }

    async fn refresh_token(&self, _user: &Username<'_>) -> Result<Token, LoginError> {
        Err(LoginError::UnsupportedOperation)
    }

    fn authenticate(&self, _header: &Token) -> Option<User<'_>> {
        None
    }
}
