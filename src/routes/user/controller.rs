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
use crate::feature;
use crate::feature::Feature;
use crate::routes::response::Page;
use crate::routes::user::error::UserError;
use crate::routes::user::request::{AdminUpdateParameters, CreateParameters, UpdateParameters};
use crate::routes::user::response::ViewableUserParameters;
use crate::security::auth::User;
use crate::security::permission::user::{
    CreateUser, DeleteAnyUser, DeleteOwnUser, EditAnyUser, EditOwnUser, ViewUser,
};
use crate::security::permission::Permission;
use crate::store::internal_user::entity::{Parameters, Username};
use crate::store::internal_user::Repository;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

/// Creates a new user within the internal authentication system.
///
/// This endpoint is unavailable and will always return HTTP status `501: Not Implemented` when an
/// authentication scheme other than the internal system is configured.
///
/// # Example Request
///
/// ```http
/// POST /v1/admin/user HTTP/1.1
/// Accept: application/json
/// Content-Type: application/json
///
/// {
///     "name": "smith",
///     "password": "hunter2!",
///     "is_password_expired": true,
///     "is_admin": true
/// }
/// ```
///
/// # Example Response
///
/// ```http
/// HTTP/1.1 200 Ok
/// Content-Type: application/json
///
/// "smith"
/// ```
#[post("/", data = "<params>")]
pub fn create(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<CreateUser>,
    store: &State<Repository>,
    user: User,
    params: Json<CreateParameters>,
) -> Result<Json<String>, UserError> {
    if !params.name.is_valid() {
        return Err(UserError::InvalidUsername("Invalid characters given"));
    }

    if !Parameters::validate_password(params.password.as_str()) {
        return Err(UserError::InsufficientPasswordStrength(
            "Expected sufficiently strong password",
        ));
    }

    if !user.has_permission(&params.permissions) {
        return Err(UserError::InsufficientGrantPermission(
            "Insufficient grant power",
        ));
    }

    let mut user = Parameters {
        display_name: params.display_name.clone(),
        password_hash: "".to_string(),
        is_password_expired: params.is_password_expired,
        permissions: params.permissions,
    };

    if let Err(e) = user.set_password(params.password.as_str()) {
        error!("Failed to set password: {}", e);
        return Err(UserError::InternalError("Failed to set password"));
    }

    match store.create(&params.name, &user) {
        Ok(_) => Ok(Json(params.name.as_str().to_string())),
        Err(e) => {
            error!("failed to create user: {}", e);
            Err(UserError::InternalError("Failed to store user"))
        }
    }
}

#[get("/")]
pub fn list_all<'a>(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<ViewUser>,
    store: &'a State<Repository>,
) -> Result<Json<Page<ViewableUserParameters<'a>>>, UserError> {
    let results = store.list().map_err(|e| {
        error!("Failed to list users: {}", e);
        UserError::InternalError("Failed to list users")
    })?;

    let users: Vec<ViewableUserParameters> = results
        .iter()
        .map(|e| ViewableUserParameters::of_internal_user(e.0.clone(), e.1.as_ref()))
        .collect();

    Ok(Json(Page {
        count: users.len(),
        items: users,
    }))
}

#[get("/me")]
pub fn get_self<'a>(user: User<'a>) -> Json<ViewableUserParameters<'a>> {
    Json(ViewableUserParameters::of_user(&user))
}

/// Retrieves all publicly visible information for a user within the internal authentication system.
///
/// This endpoint is unavailable and will always return HTTP status `501: Not Implemented` when an
/// authentication scheme other than the internal system is configured.
///
/// # Example Request
///
/// ```http
/// GET /v1/admin/user/smith HTTP/1.1
/// Accept: application/json
/// ```
///
/// # Example Response
///
/// ```http
/// HTTP/1.1 Ok
/// Content-Type: application/json
///
/// {
///     "name": "smith",
///     "is_password_expired": true,
///     "permissions": [
///         "CreateDocument",
///         "DeleteOwnDocument"
///     ]
/// }
/// ```
#[get("/<name>", rank = 2)]
pub fn get_other<'a>(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<ViewUser>,
    store: &State<Repository>,
    name: Username<'a>,
) -> Result<Json<ViewableUserParameters<'a>>, UserError> {
    if !name.is_valid() {
        return Err(UserError::InvalidUsername("Invalid name given"));
    }

    match store.get(&name) {
        Ok(Some(params)) => Ok(Json(ViewableUserParameters::of_internal_user(
            name, &params,
        ))),
        Ok(None) => Err(UserError::NoSuchUser("No such profile")),
        Err(e) => {
            error!("Failed to load user profile: {}", e);
            Err(UserError::InternalError("Failed to load profile"))
        }
    }
}

#[post("/me", data = "<params>")]
pub fn update_self<'a>(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<EditOwnUser>,
    user: User<'a>,
    store: &State<Repository>,
    params: Json<UpdateParameters>,
) -> Result<Json<ViewableUserParameters<'a>>, UserError> {
    let name = Username::new(user.id());

    if let Some(password) = params.new_password.clone() {
        if !Parameters::validate_password(password.as_str()) {
            return Err(UserError::InsufficientPasswordStrength(
                "Expected sufficiently strong password",
            ));
        }
    }

    let record = match store
        .update(&name, |record| {
            if let Some(display_name) = params.display_name.clone() {
                record.display_name = if !user.id().eq(display_name.as_str()) {
                    Some(display_name)
                } else {
                    None
                };
            }

            if let Some(password) = params.new_password.clone() {
                if let Err(e) = record.set_password(password.as_str()) {
                    // TODO: This isn't great since this error just gets swallowed
                    error!("Failed to persist password change: {}", e);
                }
            }
        })
        .map_err(|e| {
            error!("Failed to persist user update: {}", e);
            UserError::InternalError("Failed to update database")
        })? {
        Some(record) => record,
        None => return Err(UserError::InternalError("Failed to find local user")),
    };

    Ok(Json(ViewableUserParameters::of_internal_user(
        name, &record,
    )))
}

#[post("/<name>", data = "<params>", rank = 2)]
pub fn update_other<'a>(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<EditAnyUser>,
    store: &State<Repository>,
    user: User,
    name: Username<'a>,
    params: Json<AdminUpdateParameters>,
) -> Result<Json<ViewableUserParameters<'a>>, UserError> {
    if let Some(password) = params.new_password.clone() {
        if !Parameters::validate_password(password.as_str()) {
            return Err(UserError::InsufficientPasswordStrength(
                "Expected sufficiently strong password",
            ));
        }
    }

    if let Some(permissions) = params.permissions
        && !user.has_permission(&permissions)
    {
        return Err(UserError::InsufficientGrantPermission(
            "Insufficient grant power",
        ));
    }

    let record = match store
        .update(&name, |record| {
            if let Some(display_name) = params.display_name.clone() {
                record.display_name = if !name.as_str().eq(display_name.as_str()) {
                    Some(display_name)
                } else {
                    None
                };
            }

            if let Some(password) = params.new_password.clone() {
                if let Err(e) = record.set_password(password.as_str()) {
                    // TODO: This isn't great since this error just gets swallowed
                    error!("Failed to persist password change: {}", e);
                }
            }

            if let Some(permissions) = params.permissions {
                record.permissions = permissions;
            }
        })
        .map_err(|e| {
            error!("Failed to persist user update: {}", e);
            UserError::InternalError("Failed to update database")
        })? {
        Some(record) => record,
        None => return Err(UserError::InternalError("Failed to find local user")),
    };

    Ok(Json(ViewableUserParameters::of_internal_user(
        name, &record,
    )))
}

#[delete("/me")]
pub fn delete_self(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<DeleteOwnUser>,
    store: &State<Repository>,
    user: User,
) -> Result<Status, UserError> {
    store.delete(&Username::new(user.id())).map_err(|e| {
        error!("Failed to delete user from database: {}", e);
        UserError::InternalError("Failed to delete user")
    })?;

    Ok(Status::NoContent)
}

#[delete("/<name>", rank = 2)]
pub fn delete_other(
    _f: &Feature<feature::auth::InternalAuth>,
    _p: &Permission<DeleteAnyUser>,
    store: &State<Repository>,
    name: Username,
) -> Result<Status, UserError> {
    store.delete(&name).map_err(|e| {
        error!("Failed to delete user from database: {}", e);
        UserError::InternalError("Failed to delete user")
    })?;

    Ok(Status::NoContent)
}
