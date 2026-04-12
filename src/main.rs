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
#![deny(redundant_imports)]
#![deny(redundant_lifetimes)]
#![deny(unit_bindings)]
#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]

#[macro_use]
extern crate rocket;

use crate::cfg::security::AuthMethod;
use crate::cfg::ApplicationConfig;
use crate::security::token::InternalTokenProvider;
use config::{Case, Config, Environment, File};
use rocket::fairing::AdHoc;
use rocket::tokio::time::interval;
use rocket::{tokio, Build, Rocket};
use std::path::Path;
use std::time::Duration;

#[cfg(feature = "ui")]
mod assets;
mod cfg;
mod feature;
mod ratelimit;
mod routes;
mod security;
mod store;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let s = Config::builder()
        .add_source(ApplicationConfig::default())
        .add_source(File::with_name(ApplicationConfig::LOCATION).required(false))
        .add_source(
            Environment::with_prefix("safehouse")
                .separator("_")
                .convert_case(Case::Snake),
        )
        .build()
        .expect("Failed to load configuration file");

    let config = s
        .try_deserialize::<ApplicationConfig>()
        .expect("Failed to deserialize configuration file");

    let store_cfg = store::Config {
        path: Path::new(config.database.path.as_str()).to_path_buf(),
        compress: config.database.compress,
    };

    let db = store::open_database(&store_cfg).expect("Database failed initialization");

    let document_store = store::document::Repository::new(&db);
    let system_store = match store::system::Repository::new(&db) {
        Ok(store) => store,
        Err(e) => panic!("System store failed initialization: {}", e),
    };

    let mut r = rocket::build()
        .manage(config.clone())
        .manage(document_store.clone());

    match config.security.auth_method {
        AuthMethod::None => {
            r = r.manage(security::auth_none::stage());
        }
        AuthMethod::Internal => {
            let internal_user_store = store::internal_user::Repository::new(&db);
            let token_provider = InternalTokenProvider::new(system_store);

            r = r
                .manage(internal_user_store.clone())
                .attach(security::auth_internal::stage(
                    internal_user_store,
                    token_provider,
                ));
        }
    };

    r = attach_rate_limit(&config, r);
    r = attach_web_ui(r);

    r = r
        .attach(routes::stage())
        .attach(store::document::cleanup::CleanupFairing::new(
            document_store.clone(),
            Duration::from_mins(config.document.expiration_job_minutes),
        ))
        .attach(AdHoc::on_liftoff("Finalize Startup", |_| {
            Box::pin(async move {
                config.dump_to_log();
            })
        }));

    let _rocket = r.launch().await?;

    Ok(())
}

#[cfg(feature = "ratelimit")]
fn attach_rate_limit(cfg: &ApplicationConfig, rocket: Rocket<Build>) -> Rocket<Build> {
    if !cfg.ratelimit.enabled {
        return rocket;
    }

    rocket.attach(ratelimit::stage())
}

#[cfg(not(feature = "ratelimit"))]
fn attach_rate_limit(cfg: &ApplicationConfig, rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
}

#[cfg(feature = "ui")]
fn attach_web_ui(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.attach(assets::stage())
}

#[cfg(not(feature = "ui"))]
fn attach_web_ui(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
}
