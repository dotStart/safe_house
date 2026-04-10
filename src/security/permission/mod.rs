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
pub mod document;
pub mod user;

use crate::security::auth::User;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::de::{Error, Visitor};
use rocket::serde::{Deserialize, Deserializer, Serialize, Serializer};
use rocket::Request;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Not};

pub trait PermissionIndicator {
    fn permission() -> PermissionFlag;
}

pub struct Permission<I: PermissionIndicator>(PhantomData<I>);

#[rocket::async_trait]
impl<'r, I: PermissionIndicator> FromRequest<'r> for &'r Permission<I> {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<User>().await);

        if user.has_permission(&I::permission()) {
            Outcome::Success(&Permission(PhantomData))
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[macro_export]
macro_rules! permission_indicator {
    ( $x:ident ) => {
        pub struct $x;

        impl crate::security::permission::PermissionIndicator for $x {
            fn permission() -> crate::security::permission::PermissionFlag {
                crate::security::permission::PermissionFlag::$x
            }
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct PermissionFlag(u64);

#[allow(non_upper_case_globals)]
impl PermissionFlag {
    pub const ViewAnyDocument: PermissionFlag = PermissionFlag(1 << 0);
    pub const CreateDocument: PermissionFlag = PermissionFlag(1 << 1);
    pub const DeleteOwnDocument: PermissionFlag = PermissionFlag(1 << 2);
    pub const DeleteAnyDocument: PermissionFlag = PermissionFlag(1 << 3);
    pub const AllDocument: PermissionFlag = PermissionFlag(
        Self::ViewAnyDocument.0
            | Self::CreateDocument.0
            | Self::DeleteOwnDocument.0
            | Self::DeleteAnyDocument.0,
    );

    pub const ViewUser: PermissionFlag = PermissionFlag(1 << 8);
    pub const CreateUser: PermissionFlag = PermissionFlag(1 << 9);
    pub const EditOwnUser: PermissionFlag = PermissionFlag(1 << 10);
    pub const EditAnyUser: PermissionFlag = PermissionFlag(1 << 11);
    pub const DeleteOwnUser: PermissionFlag = PermissionFlag(1 << 12);
    pub const DeleteAnyUser: PermissionFlag = PermissionFlag(1 << 13);
    pub const AllUser: PermissionFlag = PermissionFlag(
        Self::ViewUser.0
            | Self::CreateUser.0
            | Self::EditOwnUser.0
            | Self::EditAnyUser.0
            | Self::DeleteOwnUser.0
            | Self::DeleteAnyUser.0,
    );

    pub const None: PermissionFlag = PermissionFlag(0);
    pub const All: PermissionFlag = PermissionFlag(Self::AllDocument.0 | Self::AllUser.0);

    #[inline]
    pub fn contains(&self, other: &PermissionFlag) -> bool {
        (other.0 & self.0) == other.0
    }
}

impl Not for PermissionFlag {
    type Output = Self;

    fn not(self) -> Self::Output {
        PermissionFlag(!self.0)
    }
}

impl BitOr for PermissionFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        PermissionFlag(self.0 | rhs.0)
    }
}

impl BitAnd for PermissionFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        PermissionFlag(self.0 & rhs.0)
    }
}

impl Serialize for PermissionFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for PermissionFlag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(PermissionFlagVisitor)
    }
}

struct PermissionFlagVisitor;

impl<'de> Visitor<'de> for PermissionFlagVisitor {
    type Value = PermissionFlag;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(PermissionFlag(v))
    }
}
