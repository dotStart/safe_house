<!--
  - This file is part of safe_house - a simple E2E encrypted file sharing utility.
  - Copyright (c) 2026 Yuki Donath <https://dotstart.tv> and other contributors
  -
  - This program is free software: you can redistribute it and/or modify
  - it under the terms of the GNU Affero General Public License as
  - published by the Free Software Foundation, either version 3 of the
  - License, or (at your option) any later version.
  -
  - This program is distributed in the hope that it will be useful,
  - but WITHOUT ANY WARRANTY; without even the implied warranty of
  - MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  - GNU Affero General Public License for more details.
  -
  - You should have received a copy of the GNU Affero General Public License
  - along with this program.  If not, see <https://www.gnu.org/licenses/>.
  -->
<script setup lang="ts">
import {reactive, ref} from "vue";
import {useRoute} from "vue-router";
import {string_to_uint8, to_buffer} from "@/models/utils.ts";
import BufferViewer from "@/components/BufferViewer.vue";
import {DateTime} from "luxon";
import {DocumentClient} from "@/api/document.ts";
import {use_http_client} from "@/api/client.ts";
import type {Owner} from "@/api/responses/common.ts";
import type {ViewableDocument} from "@/api/models/document.ts";
import type {ErrorResponse} from "@/api/responses/error.ts";
import type {StoredDocumentMetadata} from "@/api/responses/document.ts";
import ErrorView from "@/components/error/ErrorView.vue";

const currentRoute = useRoute();

interface LoadState {
  loading: boolean;
  view_guard: number | null;
  error: ErrorResponse | undefined;
}

interface DocumentState {
  readonly id: string;
  readonly owner: Owner | null;
  readonly created_at: FormattedDateTime;
  readonly expires_at: FormattedDateTime | null;
  readonly remaining_views: number | null;
  readonly content: ArrayBuffer;
}

interface FormattedDateTime {
  readonly full: string;
  readonly relative: string;
}

const state = reactive<LoadState>({
  loading: true,
  view_guard: null,
  error: undefined
});

function create_formatted_datetime(value: DateTime): FormattedDateTime {
  return {
    full: value.toLocaleString(DateTime.DATETIME_MED),
    relative: value.toRelative() || value.toLocaleString(DateTime.DATETIME_MED)
  }
}

const doc = ref<DocumentState | null>(null);

let id = currentRoute.params.id as string;
let encoded_key = currentRoute.params.key as string;

let documents = new DocumentClient(use_http_client());

async function decrypt(document: ViewableDocument) {
  let content = await document.to_document(to_buffer(string_to_uint8(encoded_key))).decrypt();

  let expires_at: FormattedDateTime | null = null;
  if (!!document.metadata.expires_at) {
    expires_at = create_formatted_datetime(document.metadata.expires_at);
  }

  doc.value = {
    id: document.metadata.id,
    owner: document.metadata.owner,
    created_at: create_formatted_datetime(document.metadata.created_at),
    expires_at: expires_at,
    remaining_views: document.metadata.remaining_views,
    content: content,
  };
}

async function view() {
  state.view_guard = null;
  state.loading = true;

  let document: ViewableDocument;
  try {
    document = await documents.view(id);
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  await decrypt(document);

  state.loading = false;
}

(async function () {
  state.loading = true;

  let metadata: StoredDocumentMetadata;
  try {
    metadata = await documents.get_metadata(id);
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  if (!metadata.remaining_views) {
    await view();
    return;
  }

  state.view_guard = metadata.remaining_views;
  state.loading = false;
})();
</script>

<style scoped lang="css">
.view-guard {
  --view-guard-color: #ff0180;

  padding: 50px;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  text-align: center;
}

.view-guard h2,
.view-guard strong {
  color: var(--view-guard-color);
}

.view-guard button {
  --pico-background-color: var(--view-guard-color);
  --pico-border-color: var(--view-guard-color);
}

.metadata .parameter > .title {
  text-decoration: underline;
  color: var(--pico-h2-color);
}
</style>

<template>
  <ErrorView :error="state.error">
    <section>
      <hgroup>
        <h2>view document</h2>
      </hgroup>

      <section>
        <template v-if="!state.view_guard && !!doc">
          <article class="grid metadata">
            <div class="column">
              <div class="parameter">
                <p class="title">owner</p>
                <p class="value" v-if="doc.owner == null">unknown</p>
                <p class="value" v-else :title="doc.owner.id">{{ doc.owner.display_name }}</p>
              </div>
              <div class="parameter">
                <p class="title">created at</p>
                <p class="value" :title="doc.created_at.full">{{ doc.created_at.relative }}</p>
              </div>
            </div>
            <div class="column">
              <div class="parameter">
                <p class="title">remaining views</p>
                <p class="value" v-if="doc.remaining_views == null">unlimited</p>
                <p class="value" v-else-if="doc.remaining_views == 0">none</p>
                <p class="value" v-else>{{ doc.remaining_views }}</p>
              </div>
              <div class="parameter">
                <p class="title">expires</p>
                <p class="value" v-if="!doc.expires_at">never</p>
                <p class="value" v-else :title="doc.expires_at.full">{{
                    doc.expires_at.relative
                  }}</p>
              </div>
            </div>
          </article>

          <hr/>
        </template>
        <article v-if="!state.view_guard">
          <BufferViewer v-if="!!doc" :buffer="doc.content"/>
          <span aria-busy="true" v-else>loading & decrypting (this may take a while) ...</span>
        </article>

        <article class="view-guard" v-else>
          <section>
            <h2>
              WARNING<br/>
              ---------
            </h2>
            <p>
              this document may only be viewed
              <strong>
                <span v-if="state.view_guard > 1">another {{ state.view_guard }} times</span>
                <span v-else>once</span><br/>
              </strong>
              are you sure you want to view it now?
            </p>
          </section>
          <button @click.prevent="view()">view</button>
        </article>
      </section>
    </section>
  </ErrorView>
</template>
