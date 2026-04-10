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
import type {ErrorResponse} from "@/api/responses/error.ts";
import {computed, reactive, ref} from "vue";
import {use_http_client} from "@/api/client.ts";
import {AuthClient} from "@/api/auth.ts";
import router from "@/router";
import {use_login} from "@/store/global.ts";
import {UserClient} from "@/api/user.ts";

let client = use_http_client();
let login = use_login();

let auth = new AuthClient(client);
let user = new UserClient(client);

interface State {
  loading: boolean,
  error: ErrorResponse | undefined,
}

interface Parameters {
  username: string,
  password: string
}

interface Validation {
  username: boolean,
  password: boolean
}

const state = reactive<State>({
  loading: false,
  error: undefined
});

const params = reactive<Parameters>({
  username: "",
  password: ""
});

const validation = computed<Validation>(() => {
  return {
    username: !!params.username && /^[A-Za-z0-9]{3,}/.test(params.username),
    password: !!params.password
  };
});

async function process_login() {
  state.loading = true;

  let token: string;
  try {
    token = await auth.login(params.username, params.password);
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  login.token = token;
  state.loading = false;

  await router.push({
    name: 'home'
  })
}
</script>

<template>
  <ErrorView :error="state.error">
    <hgroup>
      <h2>login</h2>
    </hgroup>

    <section>
      <article>
        <form>
          <fieldset>
            <label>
              username
              <input
                type="text"
                aria-describedby="username-helper"
                :aria-disabled="state.loading"
                :aria-invalid="!validation.username"
                v-model="params.username"/>
              <small id="username-helper" v-if="!validation.username">must consist of
                three or more alphanumeric characters</small>
              <small id="username-helper" v-else>your username</small>
            </label>
            <label>
              password
              <input
                type="password"
                aria-describedby="password-helper"
                :aria-disabled="state.loading"
                :aria-invalid="!validation.password"
                v-model="params.password"/>
              <small id="password-helper" v-if="!validation.password">cannot be empty</small>
              <small id="password-helper" v-else>your password</small>
            </label>
          </fieldset>

          <button
            class="outline"
            @click.prevent="process_login()"
            :aria-disabled="state.loading"
            :aria-busy="state.loading">login
          </button>
        </form>
      </article>
    </section>
  </ErrorView>
</template>
