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
use crate::store::error::StoreError;
use std::fmt::{Display, Formatter};

pub enum SystemStoreError {
    UnknownSchemaVersion(u64),
    GenericStoreError(StoreError),
}

impl Display for SystemStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemStoreError::UnknownSchemaVersion(version) => {
                f.write_fmt(format_args!("unknown system schema version:  {}", version))
            }
            SystemStoreError::GenericStoreError(e) => {
                f.write_fmt(format_args!("store error: {}", e))
            }
        }
    }
}
