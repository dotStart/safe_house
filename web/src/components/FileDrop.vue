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
import {ref, useTemplateRef} from "vue";

const ACCEPTABLE_TYPES = ["text/plain", "image/png", "image/jpeg"];

const emit = defineEmits<{
  change: [file: File | null]
}>();

let file_name = ref("");
let file_input = useTemplateRef<HTMLInputElement>("file_input");

function filter_files(items: DataTransferItemList): DataTransferItem | null {
  const filtered = [...items].filter(
    (item) => item.kind === "file",
  );

  if (filtered.length != 1) {
    return null;
  }

  const fileItem = filtered[0] as DataTransferItem;
  if (!ACCEPTABLE_TYPES.includes(fileItem.type)) {
    return null;
  }

  return fileItem;
}

function on_click(e: Event) {
  file_input.value?.click();
}

function on_drop(e: DragEvent) {
  if (!e.dataTransfer) {
    return
  }

  const file = filter_files(e.dataTransfer.items)?.getAsFile();
  if (!file) {
    return;
  }

  file_name.value = file.name;
  emit("change", file);
}

function on_drag_over(e: DragEvent) {
  if (!e.dataTransfer) {
    return
  }

  const fileItem = filter_files(e.dataTransfer.items);
  e.dataTransfer.dropEffect = !!fileItem ? "copy" : "none";
}

function on_file_change(e: Event) {
  if (!(e.target instanceof HTMLInputElement))
    return;

  let target = e.target as HTMLInputElement;
  if (!target.files || !target.files.length) {
    return;
  }

  let file = target.files.item(0);
  if (!file) {
    return;
  }

  file_name.value = file.name;
  emit("change", file);
}
</script>

<style scoped lang="css">
.drop-zone {
  padding: 10px;
  /* account for fieldset spacing */
  margin-bottom: calc(var(--pico-spacing) * 2);

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  background: rgba(255, 255, 255, 0.025);
  border-radius: var(--pico-border-radius);
  cursor: pointer;
}

.drop-zone > input {
  display: none;
}
</style>

<template>
  <div
    class="drop-zone"
    @drop.prevent="on_drop($event)"
    @dragover.prevent="on_drag_over($event)"
    @click="on_click($event)">
    <p v-if="file_name == ''">drag & drop your file here or click to browse</p>
    <template v-else>
      <p>selected file: {{ file_name }}</p>
      <small>drag & drop a different file here or click to browse</small>
    </template>
    <input
      ref="file_input"
      type="file"
      :accept="ACCEPTABLE_TYPES.join(',')"
      @change="on_file_change($event)"/>
  </div>
</template>
