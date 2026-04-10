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
import {use_server_config} from "@/store/global.ts";
import {AuthMode} from "@/api/responses/app.ts";
import NotFoundView from "@/components/error/NotFoundView.vue";

let server_config = use_server_config();
</script>

<template>
  <section>
    <nav aria-label="breadcrumb" v-if="server_config.setup">
      <ul>
        <li>
          <a>welcome</a>
        </li>
        <li v-if="server_config.auth_system == AuthMode.INTERNAL">
          <a>user setup</a>
        </li>
        <li>
          <a>done</a>
        </li>
      </ul>
    </nav>
  </section>
  <section>
    <RouterView v-if="server_config.setup"/>
    <NotFoundView v-else/>
  </section>
</template>
