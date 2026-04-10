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
pub mod auth;
pub mod system;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Orbit, Request, Rocket};
use std::marker::PhantomData;

/// Request guard which disables endpoints via returning HTTP error `501: Not Implemented`, when a
/// feature check fails.
///
/// # Example
///
/// For instance, if you want to provide a feature based on a configuration variable:
///
/// ```rust
/// pub struct MyFeature;
///
/// impl FeatureIndicator for MyFeature {
///     fn check(rocket: &Rocket<Orbit>) -> bool {
///         let cfg = match rocket::state::<AppConfig>() {
///             Some(cfg) => cfg,
///             None => return false,
///         }
///
///         cfg.some_feature.enabled
///     }
/// }
///
/// #[get("/")]
/// fn handler(_f: &Feature<MyFeature>) -> Status {
///     Status::Ok
/// }
/// ```
///
/// The resulting handler will only be invoked if `cfg.some_feature.enabled` is set to `true`.
/// Otherwise, error `501: Not Implemented` will be returned for all handler invocations.
pub struct Feature<F: FeatureIndicator>(PhantomData<F>);

#[rocket::async_trait]
impl<'r, F: FeatureIndicator> FromRequest<'r> for &'r Feature<F> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if F::check(request.rocket()) {
            Outcome::Success(&Feature(PhantomData))
        } else {
            Outcome::Forward(Status::NotImplemented)
        }
    }
}

pub struct FeatureState<F: FeatureIndicator>(bool, PhantomData<F>);

impl<F: FeatureIndicator> FeatureState<F> {
    pub fn is_available(&self) -> bool {
        self.0
    }

    pub fn is_unavailable(&self) -> bool {
        !self.0
    }
}

#[rocket::async_trait]
impl<'r, F: FeatureIndicator> FromRequest<'r> for &'r FeatureState<F> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = F::check(request.rocket());

        // FIXME: I'm clearly too tired to address this right now
        Outcome::Success(if state {
            &FeatureState::<F>(true, PhantomData)
        } else {
            &FeatureState::<F>(false, PhantomData)
        })
    }
}

/// Indicates whether a given feature is available within the current application configuration.
pub trait FeatureIndicator {
    fn check(rocket: &Rocket<Orbit>) -> bool;
}
