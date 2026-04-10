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
use crate::store::internal_user::entity::Username;
use crate::store::system::Repository;
use chrono::{Days, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct InternalTokenProvider {
    store: Repository,
}

impl InternalTokenProvider {
    pub fn new(store: Repository) -> Self {
        Self { store }
    }

    pub fn generate(&self, user: &Username) -> Result<Token<'_>, InternalTokenError> {
        let now = Utc::now();
        let secret = self
            .store
            .jwt_secret()
            .map_err(|e| InternalTokenError::SecretLoadFailure(e))?;
        let encoding_key = EncodingKey::from_secret(&secret);

        jsonwebtoken::encode(
            &Header::default(),
            &InternalClaims {
                sub: user.as_str().to_string(),
                exp: (now + Days::new(2)).timestamp() as usize,
                nbf: now.timestamp() as usize,
            },
            &encoding_key,
        )
        .map(|token| Token::of(token.as_str()))
        .map_err(|e| InternalTokenError::EncodingFailure(e.to_string()))
    }

    pub fn verify(&self, token: &Token) -> Result<InternalClaims, InternalTokenError> {
        let secret = self
            .store
            .jwt_secret()
            .map_err(|e| InternalTokenError::SecretLoadFailure(e))?;
        let decoding_key = DecodingKey::from_secret(&secret);

        let mut validation = Validation::default();
        validation.validate_aud = false;
        validation.validate_exp = true;
        validation.validate_nbf = true;

        let claims =
            jsonwebtoken::decode::<InternalClaims>(token.as_str(), &decoding_key, &validation)
                .map_err(|e| InternalTokenError::DecodingFailure(e.to_string()))?
                .claims;

        Ok(claims)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalClaims {
    pub sub: String,
    pub exp: usize,
    pub nbf: usize,
}

pub enum InternalTokenError {
    EncodingFailure(String),
    DecodingFailure(String),
    SecretLoadFailure(StoreError),
}

impl Display for InternalTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalTokenError::EncodingFailure(e) => {
                f.write_fmt(format_args!("failed to encode token: {}", e))
            }
            InternalTokenError::DecodingFailure(e) => {
                f.write_fmt(format_args!("failed to decode token: {}", e))
            }
            InternalTokenError::SecretLoadFailure(e) => {
                f.write_fmt(format_args!("failed to load secret: {}", e))
            }
        }
    }
}

pub struct Token<'a>(Cow<'a, str>);

impl Token<'_> {
    pub fn of(value: &str) -> Self {
        Token(Cow::Owned(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
