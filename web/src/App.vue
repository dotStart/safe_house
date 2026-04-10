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
import {RouterLink, RouterView} from 'vue-router'
import {ref} from "vue";
import {use_login, use_server_config} from "@/store/global.ts";
import {use_http_client} from "@/api/client.ts";
import {AppClient} from "@/api/app.ts";
import {type ApplicationInfo, AuthMode} from "@/api/responses/app.ts";
import type {ErrorResponse} from "@/api/responses/error.ts";
import ErrorView from "@/components/error/ErrorView.vue";
import {PermissionGroup} from "@/api/user.ts";

let client = use_http_client();

const app = new AppClient(client);

const isDevelopmentVersion = import.meta.env.DEV;
const server_config = use_server_config();
const login = use_login();

interface LoadingState {
  loading: boolean,
  error: ErrorResponse | undefined;
}

interface AppMetadata {
  name: string;
  version: string;
  repository: string;
}

const state = ref<LoadingState>({
  loading: true,
  error: undefined,
})
const metadata = ref<AppMetadata | null>(null);

(async function () {
  let info: ApplicationInfo;
  try {
    info = await app.info();
  } catch (e: any) {
    state.value.error = e as ErrorResponse;
    return
  }

  metadata.value = {
    name: info.name,
    version: info.version,
    repository: info.repository,
  };

  const security = info.config.security;
  server_config.setup = info.config.setup;
  server_config.auth_system = security.auth;
  server_config.key_derivation_rounds = security.key_derivation_rounds;

  state.value.loading = false;
}());
</script>

<style scoped lang="css">
header > nav {
  padding: var(--pico-typography-spacing-vertical) 0 0 0;
}

header > nav > ul > li > h1 {
  margin: 0;
}

nav > ul > li > a {
  text-decoration: underline !important;
}

nav > ul > li > a:hover {
  text-decoration: none !important;
}

header details.dropdown > summary + ul {
  right: auto;
  width: auto;
}

main {
  margin: calc(var(--pico-typography-spacing-vertical) * 2) auto;
}

footer {
  --pico-color: #8f8f8f;
}
</style>

<template>
  <header class="container">
    <nav>
      <ul>
        <li>
          <h1>safe_house</h1>
        </li>
      </ul>
      <ul>
        <li>
          <RouterLink to="/">upload</RouterLink>
        </li>
        <template v-if="server_config.auth_system == AuthMode.INTERNAL">
          <li v-if="login.logged_in && !!login.profile">
            <details class="dropdown">
              <summary :title="login.profile.username">
                {{ login.profile.display_name || login.profile.username }}
              </summary>
              <ul dir="rtl">
                <li>
                  <RouterLink to="/user/me">Settings</RouterLink>
                </li>
                <li v-if="login.has_any_permission(PermissionGroup.Admin)">
                  <RouterLink to="/admin">Admin</RouterLink>
                </li>
                <li>
                  <RouterLink to="/user/logout">Logout</RouterLink>
                </li>
              </ul>
            </details>
          </li>
          <li v-else>
            <RouterLink to="/user/login">login</RouterLink>
          </li>
        </template>
      </ul>
    </nav>
  </header>
  <hr/>
  <main class="container">
    <ErrorView :error="state.error">
      <section v-if="state.loading">
        <article>
          <span aria-busy="true">loading ...</span>
        </article>
      </section>
      <RouterView v-else/>
    </ErrorView>
  </main>
  <hr/>
  <footer class="container">
    <nav>
      <ul>
        <li>
          <small>
            made by humans with love - {{ !!metadata ? metadata.name : "safe_house" }}
            <span v-if="!!metadata"> v{{ metadata.version }}</span>
            <span v-if="isDevelopmentVersion"> (development ui)</span><br/>
            licensed under the
            <a target="_blank" href="https://www.gnu.org/licenses/agpl-3.0-standalone.html">gnu
              affero general public license (agpl)</a>
          </small>
        </li>
      </ul>
      <ul>
        <li>
          <a v-if="!!metadata" :href="metadata.repository">source code</a>
        </li>
      </ul>
    </nav>
  </footer>
</template>
