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
use crate::assets::files::StaticAssets;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[get("/")]
pub fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = StaticAssets::get("index.html")?;
    Some(RawHtml(asset.data))
}

#[get("/assets/<file..>")]
pub fn static_file(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = StaticAssets::get(format!("assets/{}", filename).as_str())?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}
