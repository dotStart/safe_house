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
import {has_any_permission, Permission, PermissionGroup, UserClient} from "@/api/user.ts";
import {use_http_client} from "@/api/client.ts";
import type {ErrorResponse} from "@/api/responses/error.ts";
import {reactive, ref} from "vue";
import type {ViewableUserParameters} from "@/api/responses/user.ts";
import {use_login} from "@/store/global.ts";

const login = use_login();
const client = use_http_client();
const users = new UserClient(client);

interface State {
  loading: boolean,
  error: ErrorResponse | undefined,
}

const state = reactive<State>({
  loading: true,
  error: undefined,
});

const listing = ref<ViewableUserParameters[]>();

(async function () {
  try {
    listing.value = (await users.list()).items;
  } catch (e: any) {
    state.error = e as ErrorResponse;
  }

  state.loading = false;
})();
</script>

<style scoped lang="css">
.grid .actions {
  display: flex;
  flex-direction: row-reverse;
  align-items: center;
}

table .actions button {
  --pico-form-element-spacing-vertical: 2px;
  --pico-form-element-spacing-horizontal: 4px;
}
</style>

<template>
  <section class="grid">
    <hgroup>
      <h2>user list</h2>
      <p>list of all currently registered users on this instance</p>
    </hgroup>
    <div class="actions">
      <RouterLink to="/admin/users/create" custom v-slot="{ navigate }">
        <button class="outline success" @click.prevent="navigate()">create user</button>
      </RouterLink>
    </div>
  </section>
  <section class="overflow-auto">
    <ErrorView :error="state.error">
      <table class="striped">
        <thead>
          <tr>
            <th scope="col">name</th>
            <th scope="col">display_name</th>
            <th scope="col">permissions</th>
            <th scope="col">actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="state.loading">
            <th colspan="4">loading ...</th>
          </tr>
          <template v-else>
            <tr v-for="user in listing">
              <th scope="row">{{ user.name }}</th>
              <td>{{ user.display_name }}</td>
              <td>
                <span
                  class="danger"
                  v-if="has_any_permission(user.permissions, PermissionGroup.Admin)">admin</span>
                <span v-else>user</span>
              </td>
              <td class="grid actions">
                <RouterLink
                  :to="{name: 'user_edit', params: { name: user.name }}"
                  custom
                  v-slot="{ navigate }">
                  <button
                    :disabled="!login.has_permission(Permission.EditAnyUser)"
                    @click.prevent="navigate()">edit
                  </button>
                </RouterLink>
                <RouterLink
                  :to="{name: 'user_delete', params: { name: user.name }}"
                  custom
                  v-slot="{ navigate }">
                  <button
                    class="danger"
                    :disabled="!login.has_permission(Permission.DeleteAnyUser)"
                    @click.prevent="navigate()">
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
