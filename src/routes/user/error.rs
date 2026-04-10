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

/// Represents all possible error scenarios within the user endpoint category.
#[derive(Responder)]
pub enum UserError {
    /// Identifies that an invalid username has been given (e.g. it contains characters which are
    /// not permitted to be present within names).
    #[response(status = 400, content_type = "json")]
    InvalidUsername(&'static str),

    /// Identifies that a password with insufficient strength was given.
    #[response(status = 400, content_type = "json")]
    InsufficientPasswordStrength(&'static str),

    /// Identifies that the user cannot grant one or more permissions given within the parameters.
    InsufficientGrantPermission(&'static str),

    /// Identifies that no such user exists.
    #[response(status = 404, content_type = "json")]
    NoSuchUser(&'static str),

    /// Identifies that an internal error (such as a database error) has occurred.
    #[response(status = 500, content_type = "json")]
    InternalError(&'static str),
}
