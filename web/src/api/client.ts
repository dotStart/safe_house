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
import axios, {type Axios, type AxiosResponse, type InternalAxiosRequestConfig} from "axios";
import {type App, inject} from "vue";
import {ErrorResponse} from "@/api/responses/error.ts";
import {use_login} from "@/store/global.ts";

const INJECTION_KEY = "http_client";

export function use_http_client(): Axios {
  return <Axios>inject(INJECTION_KEY);
}

export function create_http_client(): Axios {
  let client = axios.create({
    baseURL: import.meta.env.VITE_API_BASE || ''
  });

  client.interceptors.request.use((config: InternalAxiosRequestConfig): InternalAxiosRequestConfig => {
    const store = use_login();

    if (!!store.token) {
      config.headers.Authorization = `Bearer ${store.token}`
    }

    return config;
  });

  client.interceptors.response.use((response: AxiosResponse): AxiosResponse => response, (error: any): never => {
    const response: AxiosResponse = error.response;
    if (!response) {
      throw new ErrorResponse(0, "Service unavailable");
    }

    if (response.status == 401) {
      const store = use_login();
      store.clear();
    }

    throw new ErrorResponse(response.status, response.statusText);
  });

  return client;
}

export default function (app: App) {
  app.provide(INJECTION_KEY, create_http_client());
}
