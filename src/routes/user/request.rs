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
use crate::store::internal_user::entity::Username;
use serde::Deserialize;

/// Encapsulates the parameters necessary for creating a completely new user.
///
/// # Example Body
///
/// ```json
/// {
///     "name": "smith",
///     "display_name": "A. Smith",
///     "password": "hunter2!",
///     "is_password_expired": true,
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct CreateParameters<'a> {
    /// Human-readable alphanumeric name via which the new user shall be referenced.
    ///
    /// This value is used as an identifier within the authentication system and thus cannot be
    /// changed and must be unique within the context of a given application instance.
    pub name: Username<'a>,

    /// Display name via which this user is referenced within UI elements wherever possible.
    pub display_name: Option<String>,

    /// Plain-text version of the password via which the new user may log in.
    ///
    /// Note that the password will be stored as a salted hash within the user database after this
    /// point and will only ever be transmitted for the purposes of changing a password or logging
    /// in.
    pub password: String,

    /// Indicates whether the password has expired and must thus be changed when logging in before
    /// a full access token is granted.
    pub is_password_expired: bool,

    /// Indicates which permissions are assigned to the newly created user.
    pub permissions: PermissionFlag,
}

/// Encapsulates the parameters available when updating an existing user.
///
/// Note that all fields within this specification are optional thus permitting their omission when
/// a value does not need to be updated.
///
/// # Example Body
///
/// ```json
/// {
///     "display_name": null,
///     "new_password": "hunter3!"
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct UpdateParameters {
    /// Human-readable alphanumeric name via which the user shall be referenced going forward.
    pub display_name: Option<String>,

    /// Plain-text version of a new password which shall replace the previously stored value.
    ///
    /// Note that the password will be stored as a salted hash within the user database after this
    /// point and will only ever be transmitted for the purposes of changing a password or logging
    /// in.
    pub new_password: Option<String>,
}

/// Encapsulates the parameters available to admins when updating an existing user.
///
/// Note that all fields within this specification are optional thus permitting their omission when
/// a value does not need to be updated.
///
/// # Example Body
///
/// ```json
/// {
///     "display_name": null,
///     "new_password": "hunter3!",
///     "permissions": 18
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct AdminUpdateParameters {
    /// Human-readable alphanumeric name via which the user shall be referenced going forward.
    pub display_name: Option<String>,

    /// Plain-text version of a new password which shall replace the previously stored value.
    ///
    /// Note that the password will be stored as a salted hash within the user database after this
    /// point and will only ever be transmitted for the purposes of changing a password or logging
    /// in.
    pub new_password: Option<String>,

    /// Indicates which permissions are assigned to the user.
    pub permissions: Option<PermissionFlag>,
}
