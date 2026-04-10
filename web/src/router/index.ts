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
import {createRouter, createWebHashHistory} from 'vue-router'
import DocumentView from "@/views/document//DocumentView.vue";
import ShareView from "@/views/document//ShareView.vue";
import NotFoundView from "@/components/error/NotFoundView.vue";
import InitialUserView from "@/views/setup/InitialUserView.vue";
import SetupView from "@/views/SetupView.vue";
import WelcomeView from "@/views/setup/WelcomeView.vue";
import LoginView from "@/views/user/LoginView.vue";
import LogoutView from "@/views/user/LogoutView.vue";
import CurrentUserView from "@/views/user/CurrentUserView.vue";
import DeleteCurrentUserView from "@/views/user/DeleteCurrentUserView.vue";
import FinishSetupView from "@/views/setup/FinishSetupView.vue";
import AdminView from "@/views/AdminView.vue";
import DocumentListView from "@/views/admin/DocumentListView.vue";
import AdminHomeView from "@/views/admin/AdminHomeView.vue";
import DeleteDocumentView from "@/views/document/DeleteDocumentView.vue";
import UserListView from "@/views/admin/UserListView.vue";
import CreateUserView from "@/views/admin/CreateUserView.vue";
import EditUserView from "@/views/admin/EditUserView.vue";
import DeleteUserView from "@/views/admin/DeleteUserView.vue";
import HomeView from "@/views/HomeView.vue";

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/user/me',
      name: 'myself',
      component: CurrentUserView,
    },
    {
      path: '/user/me/delete',
      name: 'delete_myself',
      component: DeleteCurrentUserView,
    },
    {
      path: '/user/login',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/user/logout',
      name: 'logout',
      component: LogoutView
    },

    {
      path: '/document/:id/delete',
      name: 'delete',
      component: DeleteDocumentView,
    },
    {
      path: '/document/:id/share/:key',
      name: 'share',
      component: ShareView
    },
    {
      path: '/document/:id/view/:key',
      name: 'view',
      component: DocumentView,
    },

    {
      path: '/admin',
      name: 'admin',
      component: AdminView,
      children: [
        {
          path: '',
          name: 'admin_home',
          component: AdminHomeView,
        },
        {
          path: 'documents',
          name: 'document_list',
          component: DocumentListView,
        },
        {
          path: 'users',
          name: 'user_list',
          component: UserListView,
        },
        {
          path: 'users/create',
          name: 'user_create',
          component: CreateUserView,
        },
        {
          path: 'users/:name',
          name: 'user_edit',
          component: EditUserView,
        },
        {
          path: 'users/:name/delete',
          name: 'user_delete',
          component: DeleteUserView,
        }
      ]
    },

    {
      path: '/setup',
      name: 'setup',
      component: SetupView,
      children: [
        {
          path: '',
          name: 'setup_home',
          component: WelcomeView,
        },
        {
          path: 'user',
          name: 'setup_user',
          component: InitialUserView,
        },
        {
          path: 'finish',
          name: 'setup_finish',
          component: FinishSetupView,
        }
      ]
    },

    {path: '/:pathMatch(.*)*', name: 'NotFound', component: NotFoundView},
  ],
});

export default router
