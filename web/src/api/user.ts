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
import type {ViewableUserParameters} from "@/api/responses/user.ts";
import type {
  CreateParameterValidation,
  SelfParameterValidation,
  UpdateParameterValidation
} from "@/api/models/user.ts";
import zxcvbn from "zxcvbn";
import type {
  CreateUserParameters,
  UpdateSelfParameters,
  UpdateUserParameters
} from "@/api/requests/user.ts";
import type {Page} from "@/api/responses/common.ts";

export class UserClient {
  private readonly client: Axios;

  public constructor(client: Axios) {
    this.client = client;
  }

  public async list(): Promise<Page<ViewableUserParameters>> {
    return (await this.client.get<Page<ViewableUserParameters>>("v1/user")).data;
  }

  public async create(name: string, display_name: string, password: string, permissions: PermissionValue) {
    const request: CreateUserParameters = {
      name: name,
      display_name: display_name,
      password: password,
      is_password_expired: false,
      permissions: permissions
    };

    await this.client.post("v1/user", request, {
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  public async get(name: string): Promise<ViewableUserParameters> {
    return (await this.client.get<ViewableUserParameters>(`v1/user/${encodeURIComponent(name)}`)).data;
  }

  public async get_self(): Promise<ViewableUserParameters> {
    let response = await this.client.get<ViewableUserParameters>("v1/user/me");
    return response.data;
  }

  public async update(name: string, display_name: string | null, password: string | null, permissions: PermissionValue | null): Promise<ViewableUserParameters> {
    const request: UpdateUserParameters = {
      display_name: display_name,
      new_password: password,
      is_password_expired: false,
      permissions: permissions
    };

    return (await this.client.post<ViewableUserParameters>(`v1/user/${encodeURIComponent(name)}`, request, {
      headers: {
        'Content-Type': 'application/json'
      }
    })).data;
  }

  public async update_self(display_name: string | null, password: string | null): Promise<ViewableUserParameters> {
    const request: UpdateSelfParameters = {
      display_name: display_name,
      new_password: password
    }

    const response = await this.client.post<ViewableUserParameters>("v1/user/me", request, {
      headers: {
        "Content-Type": "application/json"
      }
    });

    return response.data;
  }

  public async delete(name: string) {
    await this.client.delete(`v1/user/${encodeURIComponent(name)}`)
  }

  public async delete_self() {
    await this.client.delete("v1/user/me");
  }

  public validate_create_parameters(name: string, display_name: string, password: string): CreateParameterValidation {
    const rating = zxcvbn(password);

    return {
      name: /[A-Za-z0-9]{3,}/.test(name),
      display_name: true,
      password: rating.score >= 3,
    }
  }

  public validate_update_parameters(display_name: string, password: string): UpdateParameterValidation {
    const rating = zxcvbn(password);

    return {
      display_name: true,
      password: rating.score >= 3,
    }
  }

  public validate_self_parameters(display_name: string, password: string): SelfParameterValidation {
    const rating = zxcvbn(password);

    return {
      display_name: true,
      password: password.length >= 8 && rating.score >= 3
    };
  }
}

export enum Permission {
  ViewDocument = 1 << 0,
  CreateDocument = 1 << 1,
  DeleteOwnDocument = 1 << 2,
  DeleteAnyDocument = 1 << 3,

  ViewUser = 1 << 8,
  CreateUser = 1 << 9,
  EditOwnUser = 1 << 10,
  EditAnyUser = 1 << 11,
  DeleteOwnUser = 1 << 12,
  DeleteAnyUser = 1 << 13
}

export enum PermissionGroup {
  Admin = Permission.ViewDocument | Permission.ViewUser | Permission.CreateUser | Permission.EditAnyUser | Permission.DeleteAnyUser,
  User = Permission.CreateDocument | Permission.DeleteOwnDocument | Permission.EditOwnUser | Permission.DeleteOwnUser,
  All = PermissionGroup.Admin | PermissionGroup.User | Permission.DeleteAnyDocument
}

export type PermissionKey = string;
export type PermissionValue = number;

export function has_permission(permission: PermissionValue, mask: PermissionValue): boolean {
  return (permission & mask) == mask;
}

export function has_any_permission(permissions: PermissionValue, mask: PermissionValue): boolean {
  return (permissions & mask) != 0;
}

export function permission_keys() {
  let keys: PermissionKey[] = [];

  for (const key in Permission) {
    if (!isNaN(Number(key))) {
      continue
    }

    keys.push(key);
  }

  return keys;
}

export function to_permission_list(mask: PermissionValue): PermissionKey[] {
  let permissions: PermissionKey[] = [];

  for (const key in Permission) {
    const permission_mask = Number(key);
    if (isNaN(permission_mask)) {
      continue;
    }

    if ((mask & permission_mask) == permission_mask) {
      permissions.push(Permission[key] as string);
    }
  }

  return permissions;
}

export function to_permission_mask(permissions: PermissionKey[]): PermissionValue {
  let mask: PermissionValue = 0;

  for (const key in Permission) {
    if (!isNaN(Number(key))) {
      continue
    }

    let permission_key = key as PermissionKey;
    if (permissions.indexOf(permission_key) == -1) {
      continue
    }

    mask |= Number(Permission[key]);
  }

  return mask;
}
