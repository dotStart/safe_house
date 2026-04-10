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
use rocket::serde::Deserialize;
use serde_with::base64::Base64;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadBody {
    pub key_derivation_rounds: u64,
    pub max_views: Option<u16>,
    pub expires_in: Option<u32>,
    #[serde_as(as = "Base64")]
    pub iv: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub payload: Vec<u8>,
}
