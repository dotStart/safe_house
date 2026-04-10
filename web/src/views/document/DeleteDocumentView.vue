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
import {ErrorResponse} from "@/api/responses/error.ts";
import {reactive} from "vue";
import {use_http_client} from "@/api/client.ts";
import {DocumentClient} from "@/api/document.ts";
import {useRoute} from "vue-router";
import LoadingView from "@/components/LoadingView.vue";
import router from "@/router";

const route = useRoute();
const client = use_http_client();
const documents = new DocumentClient(client);

const document_id = route.params.id as string;

interface State {
  loading: boolean;
  deleting: boolean;
  error: ErrorResponse | undefined;
}

const state = reactive<State>({
  loading: true,
  deleting: false,
  error: undefined,
});

async function delete_document() {
  state.deleting = true;

  try {
    await documents.delete(document_id);
  } catch (e: any) {
    state.error = e as ErrorResponse;
  }

  await router.push({ name: 'home' });
}

(async function () {
  try {
    let metadata = await documents.get_metadata(document_id);

    // simulate 401 if we don't have permission to delete this document
    if (!metadata.can_delete) {
      state.error = new ErrorResponse(401, "Cannot delete");
    }
  } catch (e: any) {
    state.error = e as ErrorResponse;
  }

  state.loading = false;
})();
</script>

<style scoped lang="css">
article {
  padding: 50px;

  display: flex;
  flex: 1 1 0;
  flex-direction: column;
  justify-content: center;

  text-align: center;
}

article nav {
  justify-content: space-around;
}
</style>

<template>
  <article>
    <LoadingView :loading="state.loading">
      <section>
        <h2>
          CONFIRM<br/>
          ---------
        </h2>
        <p>
          are you sure you want to delete this document?<br/>
          <strong class="danger">this cannot be undone</strong>
        </p>
      </section>
      <section>
        <nav>
          <ul>
            <li>
              <button
                class="outline"
                @click.prevent="router.back()"
                :disabled="state.deleting"
                :aria-busy="state.deleting">no, take me back
              </button>
            </li>
          </ul>
          <ul>
            <li>
              <button
                class="outline danger"
                :disabled="state.deleting"
                :aria-busy="state.deleting"
                @click.prevent="delete_document()">yes, delete
              </button>
            </li>
          </ul>
        </nav>
      </section>
    </LoadingView>
  </article>
</template>
