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
import PermissionMark from "@/components/user/PermissionMark.vue";
import {Permission} from "@/api/user.ts";

const props = defineProps({
  mask: {type: Number}
});
</script>

<style scoped lang="css">
ul {
  margin: 0;
  padding: 0;
}

ul,
ul li {
  list-style: none;
}
</style>

<template>
  <div class="permission-list grid">
    <article>
      <header>document</header>

      <ul>
        <li><PermissionMark :permission="Permission.ViewDocument" description="view all documents" :mask="props.mask">view</PermissionMark></li>
        <li><PermissionMark :permission="Permission.CreateDocument" description="create documents" :mask="props.mask">create</PermissionMark></li>
        <li><PermissionMark :permission="Permission.DeleteOwnDocument" description="delete own documents" :mask="props.mask">delete (own)</PermissionMark></li>
        <li><PermissionMark :permission="Permission.DeleteAnyDocument" description="delete any document" :mask="props.mask">delete (any)</PermissionMark></li>
      </ul>
    </article>
    <article>
      <header>user</header>

      <ul>
        <li><PermissionMark :permission="Permission.ViewUser" description="view all users" :mask="props.mask">view</PermissionMark></li>
        <li><PermissionMark :permission="Permission.CreateUser" description="create user" :mask="props.mask">create</PermissionMark></li>
        <li><PermissionMark :permission="Permission.EditOwnUser" description="edit own user" :mask="props.mask">edit (own)</PermissionMark></li>
        <li><PermissionMark :permission="Permission.EditAnyUser" description="edit any user" :mask="props.mask">edit (any)</PermissionMark></li>
        <li><PermissionMark :permission="Permission.DeleteOwnUser" description="delete own user" :mask="props.mask">delete (own)</PermissionMark></li>
        <li><PermissionMark :permission="Permission.DeleteAnyUser" description="delete any user" :mask="props.mask">delete (any)</PermissionMark></li>
      </ul>
    </article>
  </div>
</template>
