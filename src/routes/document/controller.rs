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
use crate::cfg::ApplicationConfig;
use crate::feature::auth::InternalAuth;
use crate::feature::Feature;
use crate::ratelimit;
use crate::ratelimit::guard::RateLimit;
use crate::routes::document::error::DocumentError;
use crate::routes::document::request::UploadBody;
use crate::routes::document::response::{Document, DocumentMetadata};
use crate::routes::response::{Owner, Page};
use crate::security::auth::{AuthenticationManager, User};
use crate::security::permission::document::{
    CreateDocument, DeleteAnyDocument, DeleteOwnDocument, ViewAnyDocument,
};
use crate::security::permission::{Permission, PermissionFlag};
use crate::store::document::entity::Parameters;
use crate::store::document::Repository;
use crate::store::types::Clover;
use chrono::{TimeDelta, Utc};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/", data = "<body>")]
pub fn create(
    _p: &Permission<CreateDocument>,
    _r: RateLimit<ratelimit::policy::document::CreateDocument>,
    config: &State<ApplicationConfig>,
    state: &State<Repository>,
    user: User,
    body: Json<UploadBody>,
) -> Result<Json<String>, DocumentError> {
    // TODO: Check max file size?

    if body.key_derivation_rounds != config.document.key_derivation_rounds {
        return Err(DocumentError::InvalidKeyDerivationRounds(
            "Invalid key derivation rounds",
        ));
    }

    if body.iv.len() != 12 {
        warn!("Invalid IV supplied");
        return Err(DocumentError::BadRequest("Invalid IV length"));
    }

    if body.salt.len() != 16 {
        warn!("Invalid salt supplied");
        return Err(DocumentError::BadRequest("Invalid salt length"));
    }

    let created_at = Utc::now();

    let expires_at = body
        .expires_in
        .map(|duration| created_at + TimeDelta::seconds(duration as i64));

    match state.create(&Parameters {
        owner_id: user.id(),
        created_at,
        remaining_views: body.max_views,
        expires_at,
        key_derivation_rounds: body.key_derivation_rounds,
        iv: body.iv.clone(),
        salt: body.salt.clone(),
        content: body.payload.clone(),
    }) {
        Ok(id) => {
            info!("Created new document: {}", id);
            Ok(Json(id.as_str().to_string()))
        }
        Err(e) => {
            error!("Failed to upload document: {}", e);
            Err(DocumentError::InternalError("Internal server error"))
        }
    }
}

#[get("/")]
pub fn list_all<'a>(
    _p: &Permission<ViewAnyDocument>,
    registry: &'a State<Repository>,
    user: User,
) -> Result<Json<Page<DocumentMetadata<'a>>>, DocumentError> {
    let items: Vec<DocumentMetadata<'a>> = registry
        .list()
        .map_err(|e| {
            error!("Failed to list documents: {}", e);
            DocumentError::InternalError("Failed to load documents")
        })?
        .iter()
        .map(|(id, params)| {
            let can_delete = user.has_permission(if params.owner_id.eq(user.id().as_str()) {
                &PermissionFlag::DeleteOwnDocument
            } else {
                &PermissionFlag::DeleteAnyDocument
            });

            DocumentMetadata::<'a>::of(id, params, can_delete)
        })
        .collect();

    Ok(Json(Page {
        count: items.len(),
        items,
    }))
}

#[get("/<id>/metadata")]
pub fn metadata<'a>(
    _r: RateLimit<ratelimit::policy::document::ViewDocument>,
    registry: &State<Repository>,
    id: Clover<'a>,
    user: User,
) -> Result<Json<DocumentMetadata<'a>>, DocumentError> {
    let document = match registry.get(&id) {
        Ok(Some(document)) => document,
        Ok(None) => return Err(DocumentError::NotFound("No such document")),
        Err(e) => {
            error!("Failed to retrieve document \"{}\" metadata: {}", id, e);
            return Err(DocumentError::InternalError("Internal server error"));
        }
    };

    let can_delete = user.has_permission(if document.owner_id.eq(user.id().as_str()) {
        &PermissionFlag::DeleteOwnDocument
    } else {
        &PermissionFlag::DeleteAnyDocument
    });

    Ok(Json(DocumentMetadata::of(&id, &document, can_delete)))
}

#[get("/<id>")]
pub async fn get<'a>(
    _r: RateLimit<ratelimit::policy::document::ViewDocument>,
    auth_manager: &State<AuthenticationManager>,
    registry: &State<Repository>,
    id: Clover<'a>,
) -> Result<Json<Document<'a>>, DocumentError> {
    let document = match registry.update(&id, |entry| entry.record_view()) {
        Ok(Some(document)) => document,
        Ok(None) => return Err(DocumentError::NotFound("No such document")),
        Err(e) => {
            error!("Failed to retrieve and view document \"{}\": {}", id, e);
            return Err(DocumentError::InternalError("Internal server error"));
        }
    };

    let owner = auth_manager
        .display_name(&document.owner_id)
        .await
        .map(|display_name| Owner::new(document.owner_id.clone(), display_name));

    Ok(Json(Document::of(&id, &document, owner)))
}

#[delete("/<id>")]
pub fn delete_any(
    _p: &Permission<DeleteAnyDocument>,
    registry: &State<Repository>,
    id: Clover,
) -> Result<Status, DocumentError> {
    match registry.delete(&id) {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => {
            error!("Failed to delete document \"{}\": {}", id, e);
            Err(DocumentError::InternalError("Internal server error"))
        }
    }
}

#[delete("/<id>", rank = 2)]
pub fn delete_own(
    _f: &Feature<InternalAuth>,
    _p: &Permission<DeleteOwnDocument>,
    user: User,
    registry: &State<Repository>,
    id: Clover,
) -> Result<Status, DocumentError> {
    match registry.delete_own(&id, user.id().as_str()) {
        Ok(true) => Ok(Status::NoContent),
        Ok(false) => Ok(Status::NotFound),
        Err(e) => {
            error!("Failed to delete document \"{}\": {}", id, e);
            Err(DocumentError::InternalError("Internal server error"))
        }
    }
}
