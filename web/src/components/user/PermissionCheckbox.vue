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

const model = defineModel<number>();

const props = defineProps({
  label: {type: String, required: true},
  permission: {type: Number, required: true},
  description: {type: String, required: true},
  disabled: {type: Boolean},
  mask: {type: Number},
});

const state = computed({
  get: oldValue => {
    return !!props.mask && (props.mask & props.permission) == props.permission;
  },
  set: newValue => {
    let value = model.value || 0;
    if (newValue) {
      value |= props.permission;
    } else {
      value &= ~props.permission;
    }
    model.value = value;
  }
});
</script>

<template>
  <label>
    <input type="checkbox" :disabled="disabled" v-model="state"/>
    <span :data-tooltip="description">{{ label }}</span>
  </label>
</template>
