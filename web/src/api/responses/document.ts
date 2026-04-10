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
import type {Owner} from "./common.ts";

export interface StoredDocumentMetadata {
  readonly id: string;
  readonly created_at: string;
  readonly expires_at: string | null;
  readonly remaining_views: number | null;
  readonly can_delete: boolean;
}

export interface StoredDocument {
  readonly id: string;
  readonly owner: Owner | null;
  readonly created_at: string;
  readonly remaining_views: number | null;
  readonly expires_at: string | null;
  readonly key_derivation_rounds: number;
  readonly iv: string;
  readonly salt: string;
  readonly payload: string;
}
