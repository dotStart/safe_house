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
use config::{Map, Value, ValueKind};
use nonzero_ext::nonzero;
use rocket::serde::Deserialize;
use std::num::{NonZeroU32, NonZeroU64};

#[derive(Clone, Deserialize, Debug)]
pub struct RateLimitConfig {
    pub enabled: bool,

    pub auth_login_burst: NonZeroU32,
    pub auth_login_regain_minutes: NonZeroU64,

    pub document_creations_per_hour: NonZeroU32,
    pub document_views_per_hour: NonZeroU32,
}

impl RateLimitConfig {
    pub fn dump_to_log(&self) {
        info!("   >> ratelimit.enabled: {}", self.enabled);

        info!(
            "   >> ratelimit.auth_login_burst: {}",
            self.auth_login_burst
        );
        info!(
            "   >> ratelimit.auth_login_regain_minutes: {}",
            self.auth_login_regain_minutes
        );

        info!(
            "   >> ratelimit.document_creations_per_hour: {}",
            self.document_creations_per_hour
        );
        info!(
            "   >> ratelimit.document_views_per_hour: {}",
            self.document_views_per_hour
        );
    }

    pub fn collect(&self) -> Value {
        Value::new(
            None,
            ValueKind::Table(Map::from([
                (
                    "enabled".to_string(),
                    Value::new(None, ValueKind::Boolean(self.enabled)),
                ),
                (
                    "auth_login_burst".to_string(),
                    Value::new(None, ValueKind::U64(self.auth_login_burst.get() as u64)),
                ),
                (
                    "auth_login_regain_minutes".to_string(),
                    Value::new(None, ValueKind::U64(self.auth_login_regain_minutes.get())),
                ),
                (
                    "document_creations_per_hour".to_string(),
                    Value::new(
                        None,
                        ValueKind::U64(self.document_creations_per_hour.get() as u64),
                    ),
                ),
                (
                    "document_views_per_hour".to_string(),
                    Value::new(
                        None,
                        ValueKind::U64(self.document_views_per_hour.get() as u64),
                    ),
                ),
            ])),
        )
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,

            auth_login_burst: nonzero!(8u32),
            auth_login_regain_minutes: nonzero!(10u64),

            document_creations_per_hour: nonzero!(10u32),
            document_views_per_hour: nonzero!(30u32),
        }
    }
}
