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
import ErrorView from "@/components/error/ErrorView.vue";
import {computed, reactive, ref} from "vue";
import type {ErrorResponse} from "@/api/responses/error.ts";
import {use_http_client} from "@/api/client.ts";
import {type PermissionValue, UserClient} from "@/api/user.ts";
import type {ViewableUserParameters} from "@/api/responses/user.ts";
import zxcvbn from "zxcvbn";
import PasswordStrengthMeter from "@/components/PasswordStrengthMeter.vue";
import PermissionList from "@/components/user/PermissionList.vue";

const client = use_http_client();
const users = new UserClient(client);

interface State {
  loading: boolean,
  error: ErrorResponse | undefined
}

interface Parameters {
  display_name: string,
  new_password: string,
  new_password_confirmation: string,
  permissions: PermissionValue
}

interface Validation {
  display_name: boolean,
  new_password: boolean,
  new_password_confirmation: boolean,
}

const state = reactive<State>({
  loading: true,
  error: undefined
});

const params = reactive<Parameters>({
  display_name: "",
  new_password: "",
  new_password_confirmation: "",
  permissions: 0,
});

const username = ref<string>("");

const validation = computed<Validation>(() => {
  const result = users.validate_self_parameters(params.display_name, params.new_password);

  return {
    display_name: result.display_name,
    new_password: params.new_password == "" || result.password,
    new_password_confirmation: params.new_password == params.new_password_confirmation
  };
});

const parameters_valid = computed<boolean>(() => {
  const cur = validation.value;

  return cur.display_name && cur.new_password && cur.new_password_confirmation;
})

const password_strength = computed<number>(() => {
  const new_password = params.new_password;
  return new_password != "" ? zxcvbn(new_password).score : -1;
});

async function process_update() {
  state.loading = true;

  let profile: ViewableUserParameters;
  try {
    profile = await users.update_self(
      params.display_name != "" ? params.display_name : null,
      params.new_password != "" ? params.new_password : null
    )
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  username.value = profile.name;
  params.display_name = profile.display_name;
  params.new_password = "";
  params.new_password_confirmation = "";
  params.permissions = profile.permissions;
  state.loading = false;
}

(async function () {
  let profile: ViewableUserParameters;
  try {
    profile = await users.get_self();
  } catch (e: any) {
    state.error = e as ErrorResponse;
    return
  }

  username.value = profile.name;
  params.display_name = profile.display_name;
  params.new_password = "";
  params.new_password_confirmation = "";
  params.permissions = profile.permissions;

  state.loading = false;
}());
</script>

<template>
  <ErrorView :error="state.error">
    <hgroup>
      <h2>{{ username }} ("{{ params.display_name }}")</h2>
      <p>change your own profile</p>
    </hgroup>

    <section>
      <article>
        <form>
          <fieldset>
            <label>
              username
              <input
                type="text"
                aria-describedby="username-helper"
                disabled
                :value="username"/>
              <small id="username-helper">cannot be changed</small>
            </label>
            <label>
              display name
              <input
                type="text"
                aria-describedby="display-name-helper"
                :disabled="state.loading"
                :aria-invalid="!validation.display_name"
                v-model="params.display_name"/>
              <small id="display-name-helper">name via which you will be referenced (username is
                displayed if left empty)</small>
            </label>
          </fieldset>
          <fieldset class="grid">
            <label for="password">password
              <input
                name="password"
                type="password"
                aria-describedby="password-helper"
                :aria-invalid="!validation.new_password"
                :disabled="state.loading"
                v-model="params.new_password"/>
              <small id="password-helper" v-if="params.new_password == ''">change your
                password</small>
              <small id="password-helper" v-else-if="!validation.new_password">must be at least
                eight
                characters long, contain upper and lower case characters, at least one number and
                one
                special character</small>
              <small id="password-helper" v-else-if="password_strength <= 2">insufficiently strong
                password</small>
              <small id="password-helper" v-else-if="password_strength <= 3">acceptable
                password</small>
              <small id="password-helper" v-else>strong password</small>
              <PasswordStrengthMeter :password="params.new_password" allow_empty/>
            </label>
            <label for="password_confirmation">
              confirmation
              <input
                name="password_confirmation"
                type="password"
                aria-describedby="password-confirmation-helper"
                :aria-invalid="!validation.new_password_confirmation || !validation.new_password"
                :disabled="state.loading"
                v-model="params.new_password_confirmation"/>
              <small
                id="password-confirmation-helper"
                v-if="!validation.new_password_confirmation">must
                match</small>
              <small id="password-confirmation-helper" v-else>re-enter your new chosen
                passphrase</small>
            </label>
          </fieldset>

          <nav>
            <ul>
              <li>
                <button
                  class="outline"
                  @click.prevent="process_update()"
                  :disabled="state.loading || !parameters_valid"
                  :aria-busy="state.loading">update
                </button>
              </li>
            </ul>
            <ul>
              <li>
                <RouterLink to="/user/me/delete" custom v-slot="{ navigate }">
                  <button
                    class="outline danger"
                    @click.prevent="navigate()"
                    :disabled="state.loading || !parameters_valid"
                    :aria-busy="state.loading">delete account
                  </button>
                </RouterLink>
              </li>
            </ul>
          </nav>
        </form>
      </article>
      <hr/>
      <article>
        <h3>permissions</h3>
        <PermissionList :mask="params.permissions"/>
      </article>
    </section>
  </ErrorView>
</template>
