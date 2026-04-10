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
import ErrorView from "@/components/error/ErrorView.vue";
import {reactive} from "vue";
import {ErrorResponse} from "@/api/responses/error.ts";
import {use_http_client} from "@/api/client.ts";
import {UserClient} from "@/api/user.ts";
import router from "@/router";
import {useRoute} from "vue-router";
import type {ViewableUserParameters} from "@/api/responses/user.ts";

const client = use_http_client();
const users = new UserClient(client);

let route = useRoute();
let username = route.params.name as string;

interface State {
  loading: boolean,
  error: ErrorResponse | undefined,
}

const state = reactive<State>({
  loading: true,
  error: undefined,
});

async function delete_user() {
  state.loading = true;

  try {
    await users.delete(username);
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  await router.push({name: 'user_list'});
}

(async function () {
  try {
    await users.get(username);
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
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
  <ErrorView :error="state.error">
    <hgroup>
      <h2>delete "{{ username }}"</h2>
    </hgroup>

    <section>
      <article>
        <section>
          <h2>
            WARNING<br/>
            ---------
          </h2>
          <p>
            are you sure that you want to delete this account?<br/>
            <strong class="danger">!!! this cannot be undone !!!</strong>
          </p>
        </section>
        <section>
          <nav>
            <ul>
              <li>
                <RouterLink to="/admin/users" custom v-slot="{ navigate }">
                  <button
                    class="outline"
                    @click.prevent="navigate()"
                    :disabled="state.loading"
                    :aria-busy="state.loading">cancel
                  </button>
                </RouterLink>
              </li>
            </ul>
            <ul>
              <li>
                <button
                  class="outline danger"
                  :disabled="state.loading"
                  :aria-busy="state.loading"
                  @click.prevent="delete_user()">confirm
                </button>
              </li>
            </ul>
          </nav>
        </section>
      </article>
    </section>
  </ErrorView>
</template>
