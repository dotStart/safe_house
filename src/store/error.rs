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
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum StoreError {
    CommitFailure(String),
    RollbackFailure(String),
    MissingFamily(String),
    ReadFailure(String),
    UpdateFailure(String),
    SerializationFailure(String),
    DeserializationFailure(String),
    ValidationFailure(String),
}

impl Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::CommitFailure(msg) => {
                f.write_str(format!("database commit failure: {}", msg).as_str())
            }
            StoreError::RollbackFailure(msg) => {
                f.write_str(format!("database rollback failure: {}", msg).as_str())
            }
            StoreError::MissingFamily(family) => {
                f.write_str(format!("missing database column family: {}", family).as_str())
            }
            StoreError::ReadFailure(msg) => {
                f.write_str(format!("database read failure: {}", msg).as_str())
            }
            StoreError::UpdateFailure(msg) => {
                f.write_str(format!("database update failure: {}", msg).as_str())
            }
            StoreError::SerializationFailure(msg) => {
                f.write_str(format!("entity store failure: {}", msg).as_str())
            }
            StoreError::DeserializationFailure(msg) => {
                f.write_str(format!("entity load failure: {}", msg).as_str())
            }
            StoreError::ValidationFailure(msg) => {
                f.write_str(format!("entity validation failure: {}", msg).as_str())
            }
        }
    }
}
