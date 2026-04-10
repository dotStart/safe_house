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

const props = defineProps({
  permission: {type: Number, required: true},
  description: {type: String, required: true},
  mask: {type: Number},
});

const present = computed<boolean>(() => {
  if (!props.mask) {
    return false;
  }

  return (props.mask & props.permission) == props.permission;
})
</script>

<style scoped lang="css">
.permission-mark.present {
  color: var(--safehouse-success);
}

.permission-mark.missing {
  color: var(--safehouse-danger);
}
</style>

<template>
<span class="permission-mark" :class="present ? 'present' : 'missing'">
  <template v-if="present">&check;</template>
  <template v-else>&cross;</template> <span :data-tooltip="props.description"><slot></slot></span>
</span>
</template>
