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

async function redirect_login() {
  await router.push({
    name: 'login'
  });
}
</script>

<template>
  <section>
    <hgroup>
      <h2>unauthorized</h2>
      <p>your login has expired or you have insufficient permissions to access this resource</p>
    </hgroup>
    <button class="outline" @click.prevent="redirect_login()">login</button>
  </section>
</template>
