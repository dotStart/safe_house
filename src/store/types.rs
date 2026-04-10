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
use chrono::{DateTime, Utc};
use rand::RngCore;
use rocket::request::FromParam;
use rocket::serde::de::Error;
use rocket::serde::{Deserializer, Serializer};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, UriDisplayPath, Debug)]
pub struct Clover<'a>(Cow<'a, str>);

impl Clover<'_> {
    pub fn new(time: DateTime<Utc>, sequence: u64, entropy: u64) -> Self {
        let epoch = time.timestamp_millis() as u64;
        let combined = ((epoch & 0x3FF_FFFF_FFFFu64) << 22)
            | ((sequence & 0x0FFFu64) << 10)
            | (entropy & 0x3FFu64);

        let result = base32::encode(base32::Alphabet::Z, combined.to_be_bytes().as_ref());
        Clover(Cow::Owned(result))
    }

    pub fn generate() -> Self {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let mut rng = rand::thread_rng();

        let time = Utc::now();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let entropy = rng.next_u64();

        Self::new(time, sequence, entropy)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Clover<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl FromStr for Clover<'_> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Clover(s.to_owned().into()))
    }
}

impl<'a> FromParam<'a> for Clover<'a> {
    type Error = std::convert::Infallible;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Clover(param.to_owned().into()))
    }
}

impl Serialize for Clover<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Clover<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CloverVisitor)
    }
}

struct CloverVisitor;

impl<'de> Visitor<'de> for CloverVisitor {
    type Value = Clover<'de>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Clover::<'de>(v.to_owned().into()))
    }
}
