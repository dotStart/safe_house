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
use crate::ratelimit::store::RateLimitResult;
#[cfg(feature = "ratelimit")]
use rocket::fairing::AdHoc;
#[cfg(feature = "ratelimit")]
use rocket::fairing::Info;
#[cfg(feature = "ratelimit")]
use rocket::fairing::Kind;
#[cfg(feature = "ratelimit")]
use rocket::http::Header;
#[cfg(feature = "ratelimit")]
use rocket::{Request, Response};

pub mod guard;
pub mod policy;

#[cfg(feature = "ratelimit")]
mod store;

#[cfg(feature = "ratelimit")]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Rate Limiter", |rocket| async {
        rocket.manage(store::Registry::new()).attach(Fairing)
    })
}

#[cfg(feature = "ratelimit")]
struct Fairing;

#[cfg(feature = "ratelimit")]
#[rocket::async_trait]
impl rocket::fairing::Fairing for Fairing {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limit Processor",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // this is a bit hacky but there doesn't seem to be a good way to communicate between
        // guards and response callbacks at the moment
        let result = req.local_cache(|| RateLimitResult::None);

        match result {
            RateLimitResult::Permitted(limit, remaining) => {
                res.set_header(Header::new("X-RateLimit-Limit", limit.to_string()));
                res.set_header(Header::new("X-RateLimit-Remaining", remaining.to_string()));

                if (*remaining as f32) / (*limit as f32) <= 0.25f32 {
                    res.set_header(Header::new("X-RateLimit-NearLimit", "true"));
                }
            }
            RateLimitResult::Rejected(limit, window) => {
                res.set_header(Header::new("X-RateLimit-Limit", limit.to_string()));
                res.set_header(Header::new("X-RateLimit-Remaining", "0"));
                res.set_header(Header::new(
                    "X-RateLimit-RetryAfter",
                    window.as_secs().to_string(),
                ));
            }
            RateLimitResult::None => {}
        }
    }
}
