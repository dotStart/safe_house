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
use std::process::Command;

fn main() {
    let commit_id = match Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(commit_id) => commit_id,
            Err(_) => return,
        },
        Err(_) => return,
    };

    println!("cargo:rustc-env=GIT_COMMIT_ID={}", commit_id);
}
