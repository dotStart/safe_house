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
import {fileTypeFromBuffer} from "file-type";

const props = defineProps({
  buffer: {type: ArrayBuffer},
});

const container = useTemplateRef<HTMLElement>("container");
const canvas = useTemplateRef<HTMLCanvasElement>("canvas");

const fileType = ref("");
const text = ref<string | null>(null);
const image = ref<Blob | null>(null);
const size_multiplier = ref<number>(1);

function draw_blob(): Promise<void> {
  return new Promise((resolve, reject) => {
    const blob = image.value;
    if (!blob) {
      reject("Blob not set");
      return;
    }

    const c = canvas.value;
    if (!c) {
      reject("Canvas has not been initialized");
      return;
    }

    const ctx = c.getContext("2d");
    if (!ctx) {
      reject("Failed to initialize rendering context");
      return;
    }

    const img = new Image();
    img.onload = () => {
      const rect = container.value?.getBoundingClientRect();
      const container_multiplier = !!rect ? Math.min(1, rect.width / img.width) : 1;

      console.log(rect, img.width, container_multiplier);

      c.width = img.width * size_multiplier.value * container_multiplier;
      c.height = img.height * size_multiplier.value * container_multiplier;

      ctx.drawImage(img, 0, 0, c.width, c.height);
      resolve();
    }

    img.src = URL.createObjectURL(blob);
  });
}

async function resize_canvas() {
  let current_size = size_multiplier.value;
  current_size *= 2;
  if (current_size > 4) {
    current_size = 1;
  }
  size_multiplier.value = current_size;

  await draw_blob();
}

(async function () {
  let buffer = props.buffer;
  if (!buffer) {
    fileType.value = "";
    text.value = null;
    return
  }

  const typeGuess = await fileTypeFromBuffer(buffer);
  if (!typeGuess) {
    fileType.value = "";
    text.value = new TextDecoder().decode(buffer);
    return;
  }

  image.value = new Blob([buffer], {type: typeGuess.mime});
  await draw_blob();

  text.value = "";
})();
</script>

<style scoped lang="css">
section {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>

<template>
  <section ref="container">
    <textarea readonly v-if="!!text" v-model="text" rows="30"></textarea>
    <div class="overflow-auto">
      <canvas
        ref="canvas"
        v-show="!!image"
        @click.prevent="resize_canvas()"
        @contextmenu.prevent></canvas>
    </div>
  </section>
</template>
