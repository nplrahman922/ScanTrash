import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import UserDashboard from '../views/UserDashboard.vue'
import AdminDashboard from '../views/AdminDashboard.vue'
import ScanView from '../views/ScanView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/login',
    name: 'Login',
    component: LoginView
  },
  {
    path: '/user-dashboard',
    name: 'UserDashboard',
    component: UserDashboard
  },
  {
    path: '/admin-dashboard',
    name: 'AdminDashboard',
    component: AdminDashboard
  },
  {
    path: '/scan',
    name: 'ScanView',
    component: ScanView
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router