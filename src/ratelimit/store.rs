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
use dashmap::DashMap;
use governor::clock::DefaultClock;
use governor::middleware::StateInformationMiddleware;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use std::sync::Arc;
use std::time::Duration;

pub type DefaultDirectRateLimiter =
    RateLimiter<NotKeyed, InMemoryState, DefaultClock, StateInformationMiddleware>;
pub type DefaultKeyedRateLimit =
    RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock, StateInformationMiddleware>;

pub struct Registry {
    direct_map: DashMap<String, Arc<DefaultDirectRateLimiter>>,
    keyed_map: DashMap<String, Arc<DefaultKeyedRateLimit>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            direct_map: DashMap::new(),
            keyed_map: DashMap::new(),
        }
    }

    pub fn get(&self, name: String, quota: Quota) -> Arc<DefaultDirectRateLimiter> {
        Arc::clone(
            self.direct_map
                .entry(name)
                .or_insert_with(|| {
                    Arc::new(
                        RateLimiter::direct(quota).with_middleware::<StateInformationMiddleware>(),
                    )
                })
                .value(),
        )
    }

    pub fn get_keyed(&self, name: String, quota: Quota) -> Arc<DefaultKeyedRateLimit> {
        Arc::clone(
            self.keyed_map
                .entry(name)
                .or_insert_with(|| {
                    Arc::new(
                        RateLimiter::keyed(quota).with_middleware::<StateInformationMiddleware>(),
                    )
                })
                .value(),
        )
    }
}

pub enum RateLimitResult {
    Permitted(u32, u32),
    Rejected(u32, Duration),
    None,
}
