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
import type {ErrorResponse} from "@/api/responses/error.ts";
import ErrorView from "@/components/error/ErrorView.vue";
import type {StoredDocumentMetadata} from "@/api/responses/document.ts";
import {use_http_client} from "@/api/client.ts";
import {DocumentClient} from "@/api/document.ts";
import {DateTime} from "luxon";

const client = use_http_client();
const documents = new DocumentClient(client);

interface State {
  loading: boolean,
  error: ErrorResponse | undefined,
}

const state = reactive<State>({
  loading: true,
  error: undefined,
});

const listing = ref<StoredDocumentMetadata[]>([]);

function format_date_time(time: string): string {
  return DateTime.fromISO(time).toLocaleString(DateTime.DATETIME_MED);
}

(async function () {
  try {
    listing.value = (await documents.list_all()).items;
  } catch (e: any) {
    state.error = e as ErrorResponse;
  }

  state.loading = false;
}());
</script>

<style scoped lang="css">
.actions button {
  --pico-form-element-spacing-vertical: 2px;
  --pico-form-element-spacing-horizontal: 4px;
}
</style>

<template>
  <section>
    <hgroup>
      <h2>document list</h2>
      <p>list of all currently stored documents on this instance</p>
    </hgroup>
  </section>
  <section class="overflow-auto">
    <ErrorView :error="state.error">
      <table class="striped">
        <thead>
          <tr>
            <th scope="col">document id</th>
            <th scope="col">created at</th>
            <th scope="col">expires at</th>
            <th scope="col">remaining views</th>
            <th scope="col">actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="state.loading">
            <th colspan="5">loading ...</th>
          </tr>
          <template v-else>
            <tr v-for="doc in listing">
              <th scope="row">{{ doc.id }}</th>

              <td>{{ format_date_time(doc.created_at) }}</td>

              <td v-if="!!doc.expires_at">{{ format_date_time(doc.expires_at) }}</td>
              <td v-else>none</td>

              <td v-if="doc.remaining_views == 1">once</td>
              <td v-else-if="!!doc.remaining_views">{{ doc.remaining_views }} times</td>
              <td v-else>unlimited</td>

              <td class="grid actions">
                <RouterLink
                  :to="{ name: 'delete', params: { id: doc.id } }"
                  custom
                  v-slot="{ navigate }">
                  <button class="danger" :disabled="!doc.can_delete" @click.prevent="navigate()">
                    delete
                  </button>
                </RouterLink>
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </ErrorView>
  </section>
</template>
