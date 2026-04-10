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
import {onWatcherCleanup, reactive, ref, watch} from "vue";
import {to_buffer, uint8_to_string} from "@/models/utils.ts";
import router from "@/router";
import {DateTime} from "luxon";
import FileDrop from "@/components/FileDrop.vue";
import {use_server_config} from "@/store/global.ts";
import {use_http_client} from "@/api/client.ts";
import {DocumentClient} from "@/api/document.ts";
import {EncryptedDocument} from "@/api/models/document.ts";
import type {ErrorResponse} from "@/api/responses/error.ts";
import ErrorView from "@/components/error/ErrorView.vue";

const server_config = use_server_config();
const client = use_http_client();

const documents = new DocumentClient(client);

interface State {
  uploading: boolean,
  error: ErrorResponse | undefined
}

interface Parameters {
  max_views: number,
  expiration_window: number,
  text: string,
  file: File | null
}

const state = reactive<State>({
  uploading: false,
  error: undefined
});

const parameters = reactive<Parameters>({
  max_views: 2,
  expiration_window: 604800,
  text: "",
  file: null
});

function compute_expires_at(duration: number): string {
  return DateTime.now().plus({second: duration}).toLocaleString(DateTime.DATETIME_MED);
}

const expires_at = ref(compute_expires_at(parameters.expiration_window));
watch(parameters, (newValue: Parameters) => {
  function update_value() {
    expires_at.value = compute_expires_at(newValue.expiration_window);
  }

  update_value();
  const timer = window.setInterval(() => {
    update_value();
  }, 60_000);

  onWatcherCleanup(() => {
    window.clearInterval(timer);
  });
})

async function upload_text() {
  state.uploading = true;

  await process_upload(new TextEncoder().encode(parameters.text));
}

async function upload_file() {
  let f = parameters.file;
  if (!f) {
    return
  }

  state.uploading = true;

  let payload: Uint8Array;
  try {
    payload = await read_file(f);
  } catch (e: any) {
    // TODO: Show error
    return
  }

  await process_upload(payload);
}

function read_file(f: File): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const isText = f.type.startsWith("text/");
    const reader = new FileReader();
    reader.onerror = () => reject();
    reader.onload = () => {
      const result = reader.result;

      if (isText) {
        resolve(new TextEncoder().encode(result as string))
        return;
      }

      resolve(new Uint8Array(result as ArrayBuffer))
    };

    if (isText) {
      reader.readAsText(f);
    } else {
      reader.readAsArrayBuffer(f);
    }
  });
}

async function process_upload(payload: Uint8Array) {
  let encrypted = await EncryptedDocument.encrypt(server_config.key_derivation_rounds, to_buffer(payload));

  let document: string;
  try {
    document = await documents.create(
      parameters.max_views > 0 ? parameters.max_views : null,
      parameters.expiration_window > 0 ? parameters.expiration_window : null,
      encrypted
    );
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return;
  }

  await router.push({
    name: 'share',
    params: {
      id: document,
      key: uint8_to_string(new Uint8Array(encrypted.key.key))
    }
  });
}

function change_file(e: File | null) {
  parameters.file = e;
}
</script>

<style scoped lang="css">
.upload-box {
  display: flex;
  flex-direction: column;
}

.upload-box > :not(button) {
  flex-grow: 1;
}
</style>

<template>
  <ErrorView :error="state.error">
    <section>
      <article class="grid">
        <fieldset>
          <label>number of views</label>
          <input
            type="number"
            placeholder="unlimited"
            min="0"
            v-model="parameters.max_views"
            :aria-invalid="parameters.max_views == 0"
            aria-describedby="max-view-help"/>
          <small id="max-view-help" v-if="parameters.max_views != 0">may be displayed
            {{ parameters.max_views == 1 ? "once" : `${parameters.max_views} times` }}
            before being destroyed</small>
          <small id="max-view-help" v-if="parameters.max_views == 0">may be displayed an unlimited
            number of times</small>
        </fieldset>

        <fieldset>
          <label>expiration</label>
          <select
            required
            v-model="parameters.expiration_window"
            :aria-invalid="parameters.expiration_window <= 0"
            aria-describedby="expiration-window-help">
            <option :value="0">never</option>
            <option :value="3600">1 hour</option>
            <option :value="86400">1 day</option>
            <option :value="604800">1 week</option>
            <option :value="2419200">1 month</option>
          </select>
          <small id="expiration-window-help" v-if="parameters.expiration_window > 0">may be
            displayed until
            {{ expires_at }}</small>
          <small id="expiration-window-help" v-if="parameters.expiration_window <= 0">document
            will
            not
            expire</small>
        </fieldset>
      </article>
    </section>

    <hr/>

    <section>
      <div class="grid">
        <article class="upload-box">
          <fieldset>
            <textarea
              name="plain-text"
              placeholder="insert your secret text here ..."
              rows="20"
              v-model="parameters.text"
              :disabled="state.uploading"></textarea>
          </fieldset>

          <button
            type="submit"
            class="outline"
            :disabled="state.uploading || !parameters.text"
            :aria-busy="state.uploading"
            @click.prevent="upload_text()">create
          </button>
        </article>
        <article class="upload-box">
          <FileDrop @change="change_file($event)"/>

          <button
            type="submit"
            class="outline"
            :disabled="state.uploading || !parameters.file"
            :aria-busy="state.uploading"
            @click.prevent="upload_file()">upload
          </button>
        </article>
      </div>
    </section>
  </ErrorView>
</template>
