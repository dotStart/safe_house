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
use crate::feature::FeatureIndicator;
use crate::security::auth::{AuthenticationManager, AuthenticationSystem};
use rocket::{Orbit, Rocket};

/// Feature indicator for preventing endpoints which rely on the internal authentication system from
/// being invoked when another authentication method is in use.
///
/// This indicator makes use of the AuthenticationSystem value returned by the current
/// AuthenticationProvider in order to make decisions.
pub struct InternalAuth;

impl FeatureIndicator for InternalAuth {
    fn check(rocket: &Rocket<Orbit>) -> bool {
        let auth = match rocket.state::<AuthenticationManager>() {
            Some(auth) => auth,
            None => return false,
        };

        match auth.system() {
            AuthenticationSystem::Internal => true,
            _ => false,
        }
    }
}
