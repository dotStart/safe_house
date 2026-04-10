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
use crate::security::permission::PermissionFlag;
use crate::security::token::Token;
use crate::store::internal_user::entity::Username;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use std::sync::Arc;

pub const ANONYMOUS_ID: &'static str = "anonymous";
pub const ANONYMOUS_DISPLAY_NAME: &'static str = "Anonymous";

const TOKEN_PREFIX: &'static str = "Bearer ";

pub fn stage(provider: Arc<dyn AuthenticationProvider>) -> AdHoc {
    AdHoc::on_ignite("Authentication Manager", |rocket| async {
        rocket.manage(AuthenticationManager::new(provider))
    })
}

pub struct AuthenticationManager {
    provider: Arc<dyn AuthenticationProvider>,
}

impl AuthenticationManager {
    pub fn new(provider: Arc<dyn AuthenticationProvider>) -> Self {
        AuthenticationManager { provider }
    }

    pub fn system(&self) -> AuthenticationSystem {
        self.provider.system()
    }

    pub fn anonymous(&self) -> User<'_> {
        self.provider.anonymous()
    }

    pub async fn login(
        &self,
        name: &Username<'_>,
        password: &str,
    ) -> Result<Token<'_>, LoginError> {
        self.provider.login(name, password).await
    }

    pub async fn refresh_token(&self, user: &Username<'_>) -> Result<Token<'_>, LoginError> {
        self.provider.refresh_token(user).await
    }

    pub async fn display_name(&self, user_id: &str) -> Option<String> {
        self.provider.display_name(user_id).await
    }

    pub fn authenticate(&self, token: &Token) -> Option<User<'_>> {
        self.provider.authenticate(token)
    }
}

#[rocket::async_trait]
pub trait AuthenticationProvider: Send + Sync {
    fn system(&self) -> AuthenticationSystem;

    fn anonymous(&self) -> User<'_>;

    async fn display_name(&self, user_id: &str) -> Option<String>;

    async fn login(&self, name: &Username, password: &str) -> Result<Token, LoginError>;

    async fn refresh_token(&self, user: &Username<'_>) -> Result<Token, LoginError>;

    fn authenticate(&self, header: &Token) -> Option<User<'_>>;
}

#[derive(Serialize, Debug)]
pub enum AuthenticationSystem {
    None,
    Internal,
}

impl Display for AuthenticationSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "none",
            Self::Internal => "internal",
        })
    }
}

#[derive(Debug)]
pub struct User<'a> {
    id: Cow<'a, str>,
    display_name: Option<String>,
    permissions: PermissionFlag,
}

impl<'a> User<'a> {
    pub fn new(id: &str, display_name: Option<String>, permissions: PermissionFlag) -> Self {
        User {
            id: Cow::Owned(id.to_owned()),
            display_name,
            permissions,
        }
    }

    pub fn anonymous(permissions: PermissionFlag) -> Self {
        Self::new(
            ANONYMOUS_ID,
            Some(ANONYMOUS_DISPLAY_NAME.to_string()),
            permissions,
        )
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn display_name(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.id.to_string())
    }

    pub fn permissions(&self) -> PermissionFlag {
        self.permissions
    }

    pub fn has_permission(&self, permission: &PermissionFlag) -> bool {
        self.permissions.contains(permission)
    }
}

impl Clone for User<'_> {
    fn clone(&self) -> Self {
        Self::new(
            self.id.as_ref(),
            self.display_name.clone(),
            self.permissions,
        )
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User<'r> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_manager = match request.rocket().state::<AuthenticationManager>() {
            Some(mgr) => mgr,
            None => panic!("No authentication manager in scope"),
        };
        let header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => return Outcome::Success(auth_manager.anonymous()),
        };

        let user = match header.strip_prefix(TOKEN_PREFIX) {
            Some(token) => auth_manager.authenticate(&Token::of(token)),
            // token is malformed - do not accept
            None => return Outcome::Forward(Status::Unauthorized),
        };

        match user {
            Some(user) => Outcome::Success(user),
            // invalid token - do not accept
            // TODO: Use error instead of optional
            None => Outcome::Forward(Status::Unauthorized),
        }
    }
}

pub(crate) enum LoginError {
    InvalidCredentials,
    TokenError(String),
    UnsupportedOperation,
    InternalError(String),
}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::InvalidCredentials => f.write_str("Invalid credentials given"),
            LoginError::TokenError(e) => f.write_fmt(format_args!("Token error: {}", e)),
            LoginError::UnsupportedOperation => f.write_str("Unsupported operation"),
            LoginError::InternalError(e) => f.write_fmt(format_args!("Internal error: {}", e)),
        }
    }
}
