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
import type {Axios} from "axios";
import {EncryptedPayload} from "@/api/crypto.ts";
import type {CreateDocumentParameters} from "@/api/requests/document.ts";
import type {StoredDocument, StoredDocumentMetadata} from "@/api/responses/document.ts";
import {string_to_uint8, to_buffer} from "@/models/utils.ts";
import {DateTime} from "luxon";
import {EncryptedDocument, ViewableDocument} from "@/api/models/document.ts";
import type {Page} from "@/api/responses/common.ts";

export class DocumentClient {
  private readonly client: Axios;

  public constructor(client: Axios) {
    this.client = client;
  }

  public async create(max_views: number | null, expires_in: number | null, content: EncryptedDocument): Promise<string> {
    const request: CreateDocumentParameters = {
      key_derivation_rounds: content.key.derivation_rounds,
      max_views: max_views,
      expires_in: expires_in,
      iv: content.payload.iv_string,
      salt: content.key.salt_string,
      payload: content.payload.cipher_text_string
    };

    const response = await this.client.post<string>('v1/document', request, {
      headers: {
        'Content-Type': 'application/json'
      }
    });

    return response.data;
  }

  public async list_all(): Promise<Page<StoredDocumentMetadata>> {
    return (await this.client.get<Page<StoredDocumentMetadata>>("v1/document")).data;
  }

  public async get_metadata(id: string): Promise<StoredDocumentMetadata> {
    return (await this.client.get<StoredDocumentMetadata>(`v1/document/${encodeURIComponent(id)}/metadata`)).data;
  }

  public async view(id: string): Promise<ViewableDocument> {
    const metadata = (await this.client.get<StoredDocument>(`v1/document/${encodeURIComponent(id)}`)).data;

    const content = new EncryptedPayload(
      to_buffer(string_to_uint8(metadata.iv)),
      to_buffer(string_to_uint8(metadata.payload)),
    );

    let expires_at: DateTime | null = null;
    if (metadata.expires_at != null) {
      expires_at = DateTime.fromISO(metadata.expires_at);
    }

    return new ViewableDocument(
      {
        id: metadata.id,
        owner: metadata.owner,
        created_at: DateTime.fromISO(metadata.created_at),
        remaining_views: metadata.remaining_views,
        expires_at: expires_at,
      },
      content,
      to_buffer(string_to_uint8(metadata.salt)),
      metadata.key_derivation_rounds
    );
  }

  public async delete(id: string) {
    await this.client.delete(`v1/document/${encodeURIComponent(id)}`);
  }
}
