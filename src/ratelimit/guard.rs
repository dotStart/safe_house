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
use crate::cfg::ApplicationConfig;
#[cfg(feature = "ratelimit")]
use crate::ratelimit::policy::Bucket;
use crate::ratelimit::policy::Policy;
#[cfg(feature = "ratelimit")]
use crate::ratelimit::store::RateLimitResult;
#[cfg(feature = "ratelimit")]
use crate::ratelimit::store::Registry;
#[cfg(feature = "ratelimit")]
use crate::security::auth::User;
#[cfg(feature = "ratelimit")]
use crate::security::permission::PermissionFlag;
#[cfg(feature = "ratelimit")]
use governor::clock::{Clock, DefaultClock};
#[cfg(feature = "ratelimit")]
use rocket::http::Status;
#[cfg(feature = "ratelimit")]
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::marker::PhantomData;

pub struct RateLimit<P: Policy>(PhantomData<P>);

impl<P: Policy> RateLimit<P> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

#[rocket::async_trait]
impl<'r, P: Policy> FromRequest<'r> for RateLimit<P> {
    type Error = std::convert::Infallible;

    #[cfg(feature = "ratelimit")]
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<User>().await);
        if user.has_permission(&PermissionFlag::BypassRateLimit) {
            return Outcome::Success(RateLimit::new());
        }

        let config = request
            .rocket()
            .state::<ApplicationConfig>()
            .expect("application config should be available as managed state");

        let registry = match request.rocket().state::<Registry>() {
            Some(r) => r,
            None => {
                // registry is not registered with rocket so the guard is simply skipped - this allows
                // us to add rate limiters to handler functions despite the feature being disabled
                return Outcome::Success(RateLimit::new());
            }
        };

        let quota = P::quota(&config.ratelimit);
        let outcome = match P::bucket(&config.ratelimit) {
            Bucket::None => registry.get(P::name().to_string(), quota).check(),
            Bucket::Network => {
                let limiter = registry.get_keyed(P::name().to_string(), quota);
                let remote = match request.client_ip() {
                    Some(ip) => ip.to_string(),
                    None => return Outcome::Success(RateLimit::new()),
                };

                limiter.check_key(&remote)
            }
            Bucket::User => {
                let limiter = registry.get_keyed(P::name().to_string(), quota);

                limiter.check_key(&user.id())
            }
        };

        match outcome {
            Ok(state) => {
                request.local_cache(|| {
                    RateLimitResult::Permitted(
                        state.quota().burst_size().get(),
                        state.remaining_burst_capacity(),
                    )
                });
                Outcome::Success(RateLimit::new())
            }
            Err(state) => {
                request.local_cache(|| {
                    RateLimitResult::Rejected(
                        state.quota().burst_size().get(),
                        state.wait_time_from(DefaultClock::default().now()),
                    )
                });
                Outcome::Forward(Status::TooManyRequests)
            }
        }
    }

    #[cfg(not(feature = "ratelimit"))]
    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RateLimit(PhantomData))
    }
}
