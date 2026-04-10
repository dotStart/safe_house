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
#[derive(Responder)]
pub enum UserSetupError {
    #[response(status = 400, content_type = "json")]
    InvalidUsername(&'static str),

    /// Identifies that a password with insufficient strength was given.
    #[response(status = 400, content_type = "json")]
    InsufficientPasswordStrength(String),
    #[response(status = 500, content_type = "json")]
    PasswordHashFailure(&'static str),
    #[response(status = 500, content_type = "json")]
    StoreFailure(&'static str),
}
