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
import PasswordStrengthMeter from "@/components/PasswordStrengthMeter.vue";
import {computed, reactive} from "vue";
import {use_http_client} from "@/api/client.ts";
import {PermissionGroup, type PermissionValue, UserClient} from "@/api/user.ts";
import zxcvbn from "zxcvbn";
import type {ErrorResponse} from "@/api/responses/error.ts";
import PermissionMaskPicker from "@/components/user/PermissionMaskPicker.vue";
import router from "@/router";

const client = use_http_client();

const users = new UserClient(client);

interface State {
  loading: boolean;
  error: ErrorResponse | undefined;
}

interface Params {
  name: string;
  display_name: string;
  password: string;
  password_confirmation: string;
  permissions: PermissionValue;
}

interface Validation {
  parameters: boolean;
  name: boolean;
  display_name: boolean;
  password: boolean;
  password_confirmation: boolean;
}

const state = reactive<State>({
  loading: false,
  error: undefined
});

const params = reactive<Params>({
  name: '',
  display_name: '',
  password: '',
  password_confirmation: '',
  permissions: PermissionGroup.User,
});

const password_strength = computed((): number => {
  return zxcvbn(params.password || '').score;
});

const validation = computed<Validation>(() => {
  const validation = users.validate_create_parameters(params.name, params.display_name, params.password);

  const password_valid = validation.password && password_strength.value >= 3;
  const password_repeated_correctly = params.password == params.password_confirmation;

  return {
    parameters: validation.name && validation.display_name && password_valid && password_repeated_correctly,
    name: validation.name,
    display_name: validation.display_name,
    password: password_valid,
    password_confirmation: password_repeated_correctly,
  }
});

async function create_user() {
  state.loading = true;

  try {
    await users.create(params.name, params.display_name, params.password, params.permissions)
  } catch (e: any) {
    state.error = e as ErrorResponse;
  }

  await router.push({name: 'user_list'});
}
</script>

<style scoped lang="css">
.form-actions {
  display: flex;
  flex-direction: row-reverse;
  align-items: center;
}

.form-actions button {
  --pico-form-element-spacing-vertical: 4px;
  --pico-form-element-spacing-horizontal: 16px;
}
</style>

<template>
  <section>
    <hgroup>
      <h2>create user</h2>
      <p>create another user on this instance</p>
    </hgroup>
  </section>
  <section>
    <form>
      <fieldset>
        <label>
          username *
          <input
            type="text"
            aria-describedby="name-helper"
            v-model="params.name"
            :aria-invalid="!validation.name"/>
          <small id="name-helper" v-if="!validation.name">must be alpha
            numeric and consist of at least three characters</small>
          <small id="name-helper" v-else>pick the name with which you wish to log in</small>
        </label>
      </fieldset>

      <fieldset>
        <label>
          display name
          <input
            type="text"
            aria-describedby="display-name-helper"
            :aria-invalid="!validation.display_name"
            v-model="params.display_name"/>
          <small id="display-name-helper">name which you will show up as</small>
        </label>
      </fieldset>

      <fieldset class="grid">
        <label for="password">password *
          <input
            name="password"
            type="password"
            aria-describedby="password-helper"
            v-model="params.password"
            :aria-invalid="!validation.password"/>
          <small id="password-helper" v-if="!validation.password">must be at least eight
            characters long, contain upper and lower case characters, at least one number and
            one
            special character</small>
          <small id="password-helper" v-else-if="password_strength <= 2">insufficiently strong
            password</small>
          <small id="password-helper" v-else-if="password_strength <= 3">acceptable
            password</small>
          <small id="password-helper" v-else>strong password</small>
          <PasswordStrengthMeter :password="params.password"/>
        </label>
        <label for="password_confirmation">
          confirmation *
          <input
            name="password_confirmation"
            type="password"
            aria-describedby="password-confirmation-helper"
            v-model="params.password_confirmation"
            :aria-invalid="!validation.password || !validation.password_confirmation"/>
          <small id="password-confirmation-helper" v-if="!validation.password_confirmation">must
            match</small>
          <small id="password-confirmation-helper" v-else>re-enter your chosen
            passphrase</small>
        </label>
      </fieldset>

      <div class="grid">
        <h3>permissions</h3>
        <div class="form-actions grid">
          <button
            class="outline success"
            @click.prevent="params.permissions = PermissionGroup.User">user
            preset
          </button>
          <button class="outline danger" @click.prevent="params.permissions = PermissionGroup.All">
            admin
            preset
          </button>
        </div>
      </div>
      <PermissionMaskPicker v-model="params.permissions"/>

      <button
        class="outline"
        @click.prevent="create_user()"
        :disabled="!validation.parameters || state.loading"
        :aria-busy="state.loading">
        create
      </button>
    </form>
  </section>
</template>
