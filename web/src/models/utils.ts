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
const CHUNK_SIZE = 800;

export function api_url(suffix: string) {
  let prefix = import.meta.env.VITE_API_BASE || '';
  return `${prefix}${suffix}`;
}

export function uint8_to_string(array: Uint8Array): string {
  let binary_string = array.reduce((acc, current) => acc + String.fromCharCode(current), "")
  return btoa(binary_string);
}

export function string_to_uint8(str: string): Uint8Array {
  const binary_string = atob(str);
  const array = new Uint8Array(binary_string.length);
  for (let i = 0; i < binary_string.length; ++i) {
    array[i] = binary_string.charCodeAt(i);
  }
  return array;
}

export function to_buffer(array: Uint8Array): ArrayBuffer {
  return array.buffer.slice(array.byteOffset, array.byteLength + array.byteOffset) as ArrayBuffer;
}
