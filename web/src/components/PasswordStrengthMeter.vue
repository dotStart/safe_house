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
import {computed} from "vue";
import zxcvbn from "zxcvbn";

const props = defineProps({
  password: String,
  allow_empty: Boolean,
});

const password_strength = computed<number>(() => {
  if (props.allow_empty && !props.password) {
    return -1;
  }

  return zxcvbn(props.password || '').score;
});
</script>

<style scoped lang="css">

</style>

<template>
  <progress
    :value="password_strength + 1"
    max="5"
    :class="password_strength <= 2 ? 'danger' : password_strength <= 3 ? 'warning' : 'success'"/>
</template>
