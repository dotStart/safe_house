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
import {to_buffer, uint8_to_string} from "@/models/utils.ts";

export async function import_key(material: Uint8Array): Promise<CryptoKey> {
  return crypto.subtle.importKey(
    "raw",
    to_buffer(material),
    {name: "PBKDF2"},
    false,
    ["deriveKey"]);
}

export class Cipher {
  private readonly key_derivation_rounds: number;
  private key: CryptoKey | null;

  public constructor(key_derivation_rounds: number) {
    this.key_derivation_rounds = key_derivation_rounds;

    this.key = null;
  }

  public async generate_key(): Promise<KeyMaterial> {
    console.info("Generating encryption key using ", this.key_derivation_rounds, " rounds");

    const key = crypto.getRandomValues(new Uint8Array(16));
    const salt = crypto.getRandomValues(new Uint8Array(16));

    const imported_key = await crypto.subtle.importKey(
      "raw",
      key,
      {name: "PBKDF2"},
      false,
      ["deriveKey"]);

    this.key = await crypto.subtle.deriveKey(
      {
        name: "PBKDF2",
        salt,
        // TODO: Make configurable
        iterations: this.key_derivation_rounds,
        hash: "SHA-256",
      },
      imported_key,
      {name: "AES-GCM", length: 256},
      true,
      ["encrypt", "decrypt"],
    );

    return new KeyMaterial(key, salt, this.key_derivation_rounds);
  }

  public async import_key(material: KeyMaterial) {
    console.info("Deriving encryption key using ", this.key_derivation_rounds, " rounds");

    const imported_key = await crypto.subtle.importKey(
      "raw",
      material.key,
      {name: "PBKDF2"},
      false,
      ["deriveKey"]);

    this.key = await crypto.subtle.deriveKey(
      {
        name: "PBKDF2",
        salt: material.salt,
        iterations: material.derivation_rounds,
        hash: "SHA-256",
      },
      imported_key,
      {name: "AES-GCM", length: 256},
      true,
      ["encrypt", "decrypt"],
    );
  }

  public async encrypt(payload: Uint8Array<ArrayBuffer> | ArrayBuffer): Promise<EncryptedPayload> {
    let key = this.key;
    if (!key) {
      throw "Key not generated/imported";
    }

    const iv = crypto.getRandomValues(new Uint8Array(12));
    const cipher_text = await crypto.subtle.encrypt(
      {name: "AES-GCM", iv, tagLength: 128},
      key,
      payload
    );

    return new EncryptedPayload(iv, cipher_text);
  }

  public async decrypt(payload: EncryptedPayload): Promise<ArrayBuffer> {
    let key = this.key;
    if (!key) {
      throw "Key not generated/imported";
    }

    return await crypto.subtle.decrypt(
      {name: "AES-GCM", iv: payload.iv, tagLength: 128},
      key,
      payload.cipher_text,
    );
  }
}

export class KeyMaterial {
  public readonly key: Uint8Array<ArrayBuffer> | ArrayBuffer;
  public readonly salt: Uint8Array<ArrayBuffer> | ArrayBuffer;
  public readonly derivation_rounds: number;

  public constructor(key: Uint8Array<ArrayBuffer> | ArrayBuffer, salt: Uint8Array<ArrayBuffer> | ArrayBuffer, derivation_rounds: number) {
    this.key = key;
    this.salt = salt;
    this.derivation_rounds = derivation_rounds;
  }

  public get key_array(): Uint8Array {
    if (this.key instanceof ArrayBuffer) {
      return new Uint8Array(this.key);
    }

    return this.key;
  }

  public get key_string(): string {
    return uint8_to_string(this.key_array);
  }

  public get salt_array(): Uint8Array {
    if (this.salt instanceof ArrayBuffer) {
      return new Uint8Array(this.salt);
    }

    return this.salt;
  }

  public get salt_string(): string {
    return uint8_to_string(this.salt_array);
  }
}

export class EncryptedPayload {
  public readonly iv: Uint8Array<ArrayBuffer> | ArrayBuffer;
  public readonly cipher_text: Uint8Array<ArrayBuffer> | ArrayBuffer;

  public constructor(iv: Uint8Array<ArrayBuffer> | ArrayBuffer, cipher_text: Uint8Array<ArrayBuffer> | ArrayBuffer) {
    this.iv = iv;
    this.cipher_text = cipher_text;
  }

  public get iv_array(): Uint8Array {
    if (this.iv instanceof ArrayBuffer) {
      return new Uint8Array(this.iv);
    }

    return this.iv;
  }

  public get iv_string(): string {
    return uint8_to_string(this.iv_array);
  }

  public get cipher_text_array(): Uint8Array {
    if (this.cipher_text instanceof ArrayBuffer) {
      return new Uint8Array(this.cipher_text);
    }

    return this.cipher_text;
  }

  public get cipher_text_string(): string {
    return uint8_to_string(this.cipher_text_array);
  }
}
