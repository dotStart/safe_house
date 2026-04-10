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
use crate::feature::auth::InternalAuth;
use crate::feature::FeatureIndicator;
use crate::store::internal_user::Repository;
use rocket::{Orbit, Rocket};

/// Feature indicator for preventing setup related endpoints from being accessed after the
/// application instance has already gone through initial setup.
///
/// Currently, this feature is only available when internal authentication is configured and no
/// users are present within the database, as such, it implies `InternalAuth`.
pub struct InternalUserSetup;

impl FeatureIndicator for InternalUserSetup {
    fn check(rocket: &Rocket<Orbit>) -> bool {
        if !InternalAuth::check(rocket) {
            return false;
        }

        let store = match rocket.state::<Repository>() {
            Some(store) => store,
            None => return false,
        };

        match store.count() {
            Ok(count) => count == 0,
            Err(_) => false,
        }
    }
}
