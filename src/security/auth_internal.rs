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
use crate::security::auth::{AuthenticationSystem, LoginError, User};
use crate::security::permission::PermissionFlag;
use crate::security::token::{InternalTokenProvider, Token};
use crate::store::internal_user::entity::Username;
use crate::store::internal_user::Repository;
use rocket::fairing::AdHoc;
use std::sync::Arc;

pub fn stage(store: Repository, token_provider: InternalTokenProvider) -> AdHoc {
    AdHoc::on_ignite("Internal Authentication", |rocket| async {
        rocket.attach(auth::stage(AuthenticationProvider::new(
            store,
            token_provider,
        )))
    })
}

pub struct AuthenticationProvider {
    store: Repository,
    token_provider: InternalTokenProvider,
}

impl AuthenticationProvider {
    fn new(store: Repository, token_provider: InternalTokenProvider) -> Arc<Self> {
        Arc::new(AuthenticationProvider {
            store,
            token_provider,
        })
    }
}

#[rocket::async_trait]
impl auth::AuthenticationProvider for AuthenticationProvider {
    fn system(&self) -> AuthenticationSystem {
        AuthenticationSystem::Internal
    }

    fn anonymous(&self) -> User<'_> {
        // TODO: Configurable default permissions
        User::anonymous(PermissionFlag::None)
    }

    async fn display_name(&self, user_id: &str) -> Option<String> {
        if let Ok(Some(params)) = self.store.get(&Username::new(user_id.to_string())) {
            params.display_name
        } else {
            None
        }
    }

    async fn login(&self, name: &Username, password: &str) -> Result<Token, LoginError> {
        let user = match self.store.get(name) {
            Ok(Some(usr)) => usr,
            Ok(None) => return Err(LoginError::InvalidCredentials),
            Err(e) => {
                error!("Failed to login internal user: {}", e);
                return Err(LoginError::InternalError(e.to_string()));
            }
        };

        if !user.check_password(password) {
            return Err(LoginError::InvalidCredentials);
        }

        match self.token_provider.generate(name) {
            Ok(token) => Ok(token),
            Err(e) => Err(LoginError::TokenError(e.to_string())),
        }
    }

    async fn refresh_token(&self, user: &Username<'_>) -> Result<Token, LoginError> {
        match self.token_provider.generate(user) {
            Ok(token) => Ok(token),
            Err(e) => {
                error!("Failed to issue token: {}", e);
                return Err(LoginError::TokenError(e.to_string()));
            }
        }
    }

    fn authenticate(&self, header: &Token) -> Option<User<'_>> {
        let claims = match self.token_provider.verify(header) {
            Ok(claims) => claims,
            Err(e) => {
                warn!("Encountered invalid token: {}", e);
                return None;
            }
        };

        let name = Username::new(claims.sub);
        if !name.is_valid() {
            error!("Encountered token with invalid username: \"{}\"", name);
            return None;
        }

        let user = match self.store.get(&name) {
            Ok(Some(params)) => params,
            Ok(None) => {
                warn!("Encountered token for non-existent user: \"{}\"", name);
                return None;
            }
            Err(e) => {
                error!("Failed to load user \"{}\": {}", name, e);
                return None;
            }
        };

        Some(User::new(
            name.as_str(),
            user.display_name,
            user.permissions,
        ))
    }
}
