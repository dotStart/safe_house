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
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

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

macro_rules! declare_permissions {
    (@generate $idx:expr, $final:ident, ) => {
        pub const $final: PermissionFlag = PermissionFlag(1 << ($idx));
    };
    (@generate $idx:expr, $head:ident, $($tail:ident,)*) => {
        pub const $head: PermissionFlag = PermissionFlag(1 << ($idx));
        declare_permissions!(@generate $idx + 1, $($tail,)*);
    };
    ($offset:expr, $($name:ident),*) => {
        declare_permissions!(@generate $offset, $($name,)*);
    };
}

macro_rules! declare_permission_aggregator {
    ($name:ident, $($member:ident),*) => {
        pub const $name: PermissionFlag = PermissionFlag(
            $(
                Self::$member.0 |
            )* 0u64
        );
    };
}

macro_rules! declare_permission_group {
    ($group:ident, $offset:expr, $($member:ident),*) => {
        declare_permissions!($offset $(,$member)*);
        declare_permission_aggregator!($group $(,$member)*);
    };
}

#[allow(non_upper_case_globals)]
impl PermissionFlag {
    declare_permission_group!(
        AllDocument,
        0,
        ViewAnyDocument,
        CreateDocument,
        DeleteOwnDocument,
        DeleteAnyDocument
    );
    declare_permission_group!(
        AllUser,
        8,
        ViewUser,
        CreateUser,
        EditOwnUser,
        EditAnyUser,
        DeleteOwnUser,
        DeleteAnyUser
    );
    declare_permission_group!(AllSystem, 24, BypassRateLimit);

    pub const None: PermissionFlag = PermissionFlag(0);
    declare_permission_aggregator!(
        Admin,
        ViewAnyDocument,
        DeleteAnyDocument,
        ViewUser,
        CreateUser,
        EditAnyUser,
        DeleteAnyUser,
        AllSystem
    );
    declare_permission_aggregator!(Safe, AllDocument, AllUser);
    declare_permission_aggregator!(All, AllDocument, AllUser, AllSystem);

    #[inline]
    pub const fn contains(&self, other: &PermissionFlag) -> bool {
        (other.0 & self.0) == other.0
    }

    #[inline]
    pub const fn contains_any(&self, other: &PermissionFlag) -> bool {
        (other.0 & self.0) != 0
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

impl BitOrAssign for PermissionFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitAnd for PermissionFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        PermissionFlag(self.0 & rhs.0)
    }
}

impl BitAndAssign for PermissionFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
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

impl Visitor<'_> for PermissionFlagVisitor {
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
