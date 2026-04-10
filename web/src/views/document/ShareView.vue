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
import router from "@/router";
import {useRoute} from "vue-router";
import {use_http_client} from "@/api/client.ts";
import {DocumentClient} from "@/api/document.ts";
import {ref} from "vue";
import {ErrorResponse} from "@/api/responses/error.ts";
import ErrorView from "@/components/error/ErrorView.vue";

const route = useRoute();
const client = use_http_client();

const documents = new DocumentClient(client);

const document_id = route.params.id as string;
const document_key = route.params.key as string;

const link = ref<string | null>(null);
const error = ref<ErrorResponse | undefined>(undefined);

(async function () {
  try {
    await documents.get_metadata(document_id);
  } catch (e: any) {
    error.value = e as ErrorResponse;
    return
  }

  const view_route = router.resolve({
    name: 'view',
    params: {id: document_id, key: document_key}
  });

  link.value = new URL(view_route.href, window.location.origin).href;
}());

function copyLink() {
  navigator.clipboard.writeText(link.value || '');
}
</script>

<template>
  <ErrorView :error="error">
    <hgroup>
      <h2>share document</h2>
      <p>link to this document</p>
    </hgroup>

    <section>
      <article>
        <template v-if="link">
          <p>you may use the following link to forward this document:</p>

          <form>
            <fieldset role="group">
              <input type="text" readonly v-model="link"/>
              <button @click.prevent="copyLink()">copy</button>
            </fieldset>
          </form>
        </template>
        <template v-else>
          <span aria-busy="true">loading ...</span>
        </template>
      </article>
    </section>
  </ErrorView>
</template>
