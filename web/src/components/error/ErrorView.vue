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
import InternalErrorView from "@/components/error/InternalErrorView.vue";
import NotFoundView from "@/components/error/NotFoundView.vue";
import {ErrorResponse} from "@/api/responses/error.ts";
import UnauthorizedView from "@/components/error/UnauthorizedView.vue";
import ServiceUnavailableView from "@/components/error/ServiceUnavailableView.vue";

const props = defineProps({
  error: {type: ErrorResponse, required: false},
});
</script>

<template>
  <template v-if="!!props.error">
    <NotFoundView v-if="props.error.code == 404"/>
    <UnauthorizedView v-else-if="props.error.code == 401"/>
    <ServiceUnavailableView v-else-if="props.error.code == 0"/>
    <InternalErrorView v-else :message="props.error.message"/>
  </template>
  <template v-else>
    <slot></slot>
  </template>
</template>
