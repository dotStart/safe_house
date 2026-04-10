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
use crate::routes::response::Owner;
use crate::store::document::entity::Parameters;
use crate::store::types::Clover;
use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use serde_with::base64::Base64;
use serde_with::serde_as;

#[serde_as]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Document<'a> {
    pub id: Clover<'a>,
    pub owner: Option<Owner>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub remaining_views: Option<u16>,
    pub key_derivation_rounds: u64,
    #[serde_as(as = "Base64")]
    pub iv: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub payload: Vec<u8>,
}

impl<'a> Document<'a> {
    pub fn of(id: &Clover<'a>, params: &Parameters, owner: Option<Owner>) -> Self {
        Document {
            id: id.clone(),
            owner,
            created_at: params.created_at,
            expires_at: params.expires_at,
            remaining_views: params.remaining_views,
            key_derivation_rounds: params.key_derivation_rounds,
            iv: params.iv.clone(),
            salt: params.salt.clone(),
            payload: params.content.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DocumentMetadata<'a> {
    pub id: Clover<'a>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub remaining_views: Option<u16>,
    pub can_delete: bool,
}

impl<'a> DocumentMetadata<'a> {
    pub fn of(id: &Clover<'a>, params: &Parameters, can_delete: bool) -> Self {
        DocumentMetadata {
            id: id.clone(),
            created_at: params.created_at,
            expires_at: params.expires_at,
            remaining_views: params.remaining_views,
            can_delete,
        }
    }
}
