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

pub struct CreateDocument;

impl Policy for CreateDocument {
    fn name() -> &'static str {
        "create_document"
    }

    #[cfg(feature = "ratelimit")]
    fn quota(cfg: &RateLimitConfig) -> Quota {
        Quota::per_hour(cfg.document_creations_per_hour)
    }
}

pub struct ViewDocument;

impl Policy for ViewDocument {
    fn name() -> &'static str {
        "view_document"
    }

    #[cfg(feature = "ratelimit")]
    fn quota(cfg: &RateLimitConfig) -> Quota {
        Quota::per_hour(cfg.document_views_per_hour)
    }
}
