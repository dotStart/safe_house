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
import type {Axios} from "axios";
import type {UserSetupParameters} from "@/api/requests/setup.ts";
import type {InitialUserValidation} from "@/api/models/setup.ts";

export class SetupClient {
  private readonly client: Axios;

  public constructor(client: Axios) {
    this.client = client;
  }

  public validate_initial_user(name: string, display_name: string | null, password: string): InitialUserValidation {
    return {
      name: /^[A-Za-z0-9_]{3,}/.test(name),
      display_name: true,
      password: password.length >= 8 &&
        /[A-Z]/.test(password) &&
        /[a-z]/.test(password) &&
        /[0-9]/.test(password) &&
        /\W/.test(password)
    }
  }

  public async create_initial_user(name: string, display_name: string | null, password: string) {
    const request: UserSetupParameters = {
      name: name,
      display_name: display_name,
      password: password,
    };

    await this.client.post("v1/setup/user", request, {
      headers: {
        "Content-Type": "application/json",
      }
    });
  }
}
