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
import {use_login} from "@/store/global.ts";
import {Permission} from "@/api/user.ts";

const login = use_login();
</script>

<style scoped lang="css">
.panel {
  display: flex;
}

.panel > .sidebar {
  width: 25%;
  max-width: 250px;
  flex-shrink: 0;
  margin: 0 var(--pico-block-spacing-horizontal) 0 0;
}

.panel > .content {
  flex-grow: 1;
  margin: 0 0 0 var(--pico-block-spacing-horizontal);
}
</style>

<template>
  <section>
    <hgroup>
      <h2>system administration</h2>
      <p>access various system configuration options and restricted functionality</p>
    </hgroup>
  </section>
  <section class="panel">
    <article class="sidebar">
      <section>
        <aside>
          <nav>
            <ul>
              <li>
                <RouterLink to="/admin" activeClass="secondary">home</RouterLink>
              </li>
              <li v-if="login.has_permission(Permission.ViewDocument)">
                <RouterLink to="/admin/documents" activeClass="secondary">documents</RouterLink>
              </li>
              <li v-if="login.has_permission(Permission.ViewUser)">
                <RouterLink to="/admin/users" activeClass="secondary">users</RouterLink>
              </li>
            </ul>
          </nav>
        </aside>
      </section>
    </article>
    <article class="content">
      <RouterView/>
    </article>
  </section>
</template>
