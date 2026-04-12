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
import PermissionCheckbox from "@/components/user/PermissionCheckbox.vue";
import {Permission} from "@/api/user.ts";

const props = defineProps({
  disabled: {type: Boolean}
});

const model = defineModel<number>();
</script>

<template>
  <section class="grid">
    <article>
      <header>documents</header>

      <fieldset>
        <PermissionCheckbox
          label="view"
          description="view all documents"
          :permission="Permission.ViewDocument"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="create"
          description="create documents"
          :permission="Permission.CreateDocument"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="delete (own)"
          description="delete owned documents"
          :permission="Permission.DeleteOwnDocument"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="delete (any)"
          description="delete all documents regardless of owner"
          :permission="Permission.DeleteAnyDocument"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
      </fieldset>
    </article>
    <article>
      <header>users</header>

      <fieldset>
        <PermissionCheckbox
          label="view"
          description="view all users"
          :permission="Permission.ViewUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="create"
          description="create users"
          :permission="Permission.CreateUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="edit (own)"
          description="edit own user"
          :permission="Permission.EditOwnUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="edit (any)"
          description="edit all users"
          :permission="Permission.EditAnyUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="delete (own)"
          description="delete own user"
          :permission="Permission.DeleteOwnUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
        <PermissionCheckbox
          label="delete (any)"
          description="delete all users regardless of ownership"
          :permission="Permission.DeleteAnyUser"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
      </fieldset>
    </article>
    <article>
      <header>system</header>

      <fieldset>
        <PermissionCheckbox
          label="bypass ratelimit"
          description="bypass endpoint rate limits"
          :permission="Permission.BypassRateLimit"
          :disabled="disabled"
          :mask="modelValue"
          v-model="model"/>
      </fieldset>
    </article>
  </section>
</template>
