/*
 * This file is part of safe_house - a simple E2E encrypted file sharing utility.
 * Copyright (c) 2026 Yuki Donath <https://dotstart.tv> and other contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
import {defineStore} from "pinia";
import {computed, ref, watch} from "vue";
import {AuthMode} from "@/api/responses/app.ts";
import {computedAsync} from "@vueuse/core";
import {create_http_client} from "@/api/client.ts";
import {type PermissionValue, UserClient} from "@/api/user.ts";
import type {ErrorResponse} from "@/api/responses/error.ts";

const TOKEN_LOCAL_STORAGE_KEY = 'login_token';

export const use_server_config = defineStore("server_config", () => {
  const setup = ref(false);
  const auth_system = ref<AuthMode>(AuthMode.NONE);

  // we assume sensible defaults here, but realistically we should never end up in this situation
  // since the UI does not show anything if the initial configuration process fails
  const key_derivation_rounds = ref<number>(600_000);

  return {setup, auth_system, key_derivation_rounds};
});

export interface Myself {
  readonly username: string;
  readonly display_name: string | null;
  readonly permissions: number;
}

export const use_login = defineStore("login", () => {
  const token = ref<string | null>(localStorage.getItem(TOKEN_LOCAL_STORAGE_KEY));

  const logged_in = computed<boolean>(() => {
    return !!token.value;
  })

  const profile = computedAsync<Myself>(async (): Promise<Myself> => {
    // FIXME: This really sucks
    // required to set a dependency on token
    // noinspection JSUnusedLocalSymbols
    const t = token.value;

    const client = create_http_client();
    const user = new UserClient(client);

    try {
      const profile = await user.get_self();
      return {
        username: profile.name,
        display_name: profile.display_name,
        permissions: profile.permissions,
      }
    } catch (e: any) {
      let error = e as ErrorResponse;

      console.error("Failed to load user profile", e);
      if (error.code == 401) {
        clear();
      }

      return {
        username: "error",
        display_name: "error",
        permissions: 0,
      };
    }
  });

  function has_permission(permission: PermissionValue): boolean {
    return !!profile.value && (profile.value.permissions & permission) == permission;
  }

  function has_any_permission(permission: PermissionValue): boolean {
    return !!profile.value && (profile.value.permissions & permission) != 0;
  }

  function clear() {
    token.value = null;
  }

  watch(token, (newValue) => {
    if (newValue == null) {
      localStorage.removeItem(TOKEN_LOCAL_STORAGE_KEY);
      return;
    }

    localStorage.setItem(TOKEN_LOCAL_STORAGE_KEY, newValue);
  });

  return {token, logged_in, profile, has_permission, has_any_permission, clear};
})
