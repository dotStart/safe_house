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
use crate::routes::response::ErrorMetadata;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;

mod app;
mod auth;
mod document;
mod health;
mod response;
mod setup;
mod user;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Application Routes", |rocket| async {
        rocket
            .attach(app::stage())
            .attach(auth::stage())
            .attach(document::stage())
            .attach(health::stage())
            .attach(setup::stage())
            .attach(user::stage())
            .register("/v1", catchers![api_error_catcher])
    })
}

/// Provides basic JSON responses in case of error instead of the fancier HTML responses rocket
/// would otherwise return.
#[catch(default)]
fn api_error_catcher(status: Status, _request: &Request) -> Json<ErrorMetadata> {
    Json(ErrorMetadata {
        code: status.code,
        message: status.reason_lossy(),
    })
}
