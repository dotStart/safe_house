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
import {Cipher, EncryptedPayload, KeyMaterial} from "@/api/crypto.ts";
import type {Owner} from "@/api/responses/common.ts";
import type {DateTime} from "luxon";

export class EncryptedDocument {
  public readonly key: KeyMaterial;
  public readonly payload: EncryptedPayload;

  public constructor(key: KeyMaterial, payload: EncryptedPayload) {
    this.key = key;
    this.payload = payload;
  }

  public static async encrypt(key_derivation_rounds: number, payload: Uint8Array<ArrayBuffer> | ArrayBuffer) {
    const cipher = new Cipher(key_derivation_rounds);
    const key = await cipher.generate_key();
    const encrypted = await cipher.encrypt(payload);

    return new EncryptedDocument(key, encrypted);
  }

  public async decrypt(): Promise<ArrayBuffer> {
    const cipher = new Cipher(this.key.derivation_rounds);
    await cipher.import_key(this.key);
    return await cipher.decrypt(this.payload);
  }
}

export class ViewableDocument {
  public readonly metadata: DocumentMetadata;
  public readonly payload: EncryptedPayload;

  private readonly salt: ArrayBuffer;
  private readonly derivation_rounds: number;

  constructor(metadata: DocumentMetadata, payload: EncryptedPayload, salt: ArrayBuffer, derivation_rounds: number) {
    this.metadata = metadata;
    this.payload = payload;

    this.salt = salt;
    this.derivation_rounds = derivation_rounds;
  }

  public to_document(key: ArrayBuffer): EncryptedDocument {
    let key_material = new KeyMaterial(
      key,
      this.salt,
      this.derivation_rounds
    );

    return new EncryptedDocument(key_material, this.payload);
  }
}

export interface DocumentMetadata {
  readonly id: string;
  readonly owner: Owner | null;
  readonly created_at: DateTime;
  readonly remaining_views: number | null;
  readonly expires_at: DateTime | null;
}
