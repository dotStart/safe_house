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
#[cfg(feature = "ratelimit")]
use crate::cfg::ratelimit::RateLimitConfig;
use crate::ratelimit::policy::Policy;
#[cfg(feature = "ratelimit")]
use governor::Quota;
use std::time::Duration;

pub struct Login;

impl Policy for Login {
    fn name() -> &'static str {
        "auth_login"
    }

    #[cfg(feature = "ratelimit")]
    fn quota(cfg: &RateLimitConfig) -> Quota {
        Quota::with_period(Duration::from_mins(cfg.auth_login_regain_minutes.get()))
            .unwrap() // cannot fail since cfg uses NonZero
            .allow_burst(cfg.auth_login_burst)
    }
}
