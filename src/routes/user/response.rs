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
use crate::security::auth::User;
use crate::security::permission::PermissionFlag;
use crate::store::internal_user::entity::{Parameters, Username};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ViewableUserParameters<'a> {
    pub name: Username<'a>,
    pub display_name: Option<String>,
    pub permissions: PermissionFlag,
}

impl<'a> ViewableUserParameters<'a> {
    pub fn of_internal_user(name: Username<'a>, params: &Parameters) -> Self {
        ViewableUserParameters {
            name,
            display_name: params.display_name.clone(),
            permissions: params.permissions,
        }
    }

    pub fn of_user(user: &User) -> Self {
        Self {
            name: Username::new(user.id()),
            display_name: Some(user.display_name()),
            permissions: user.permissions(),
        }
    }
}
