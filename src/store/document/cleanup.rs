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
use crate::store::document::Repository;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::tokio::select;
use rocket::tokio::time::interval;
use rocket::{tokio, Orbit, Rocket};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub struct CleanupFairing {
    store: Repository,
    task_period: Duration,
    token: CancellationToken,
}

impl CleanupFairing {
    pub fn new(store: Repository, task_period: Duration) -> Self {
        Self {
            store,
            task_period,
            token: CancellationToken::new(),
        }
    }

    async fn cleanup_loop(store: Repository, task_period: Duration, token: CancellationToken) {
        let mut i = interval(task_period);
        loop {
            select! {
                _ = token.cancelled() => {
                    return;
                }
                _ = i.tick() => {
                    info!("Removing expired documents ...");
                    match store.expire() {
                        Ok(count) => {
                            info!("Deleted {} expired documents", count);
                        }
                        Err(e) => {
                            warn!("Failed to delete expired documents: {}", e);
                        }
                    }
                }
            }
        }
    }
}

#[rocket::async_trait]
impl Fairing for CleanupFairing {
    fn info(&self) -> Info {
        Info {
            name: "Document Cleanup Task",
            kind: Kind::Liftoff | Kind::Shutdown,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        let store = self.store.clone();
        let task_period = self.task_period.clone();
        let token = self.token.clone();

        tokio::spawn(async move {
            Self::cleanup_loop(store, task_period, token).await;
        });
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        self.token.cancel();
    }
}
