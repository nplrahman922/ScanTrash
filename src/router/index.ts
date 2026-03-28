import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '../stores/authStore'
import LoginView from '../views/LoginView.vue'
import UserDashboard from '../views/UserDashboard.vue'
import AdminDashboard from '../views/AdminDashboard.vue'
import ScanView from '../views/ScanView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Login',
    component: LoginView,
    meta: { guestOnly: true }
  },
  {
    path: '/user-dashboard',
    name: 'UserDashboard',
    component: UserDashboard,
    meta: { requiresAuth: true }
  },
  {
    path: '/admin-dashboard',
    name: 'AdminDashboard',
    component: AdminDashboard,
    meta: { requiresAuth: true }
  },
  {
    path: '/scan',
    name: 'Scan',
    component: ScanView,
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

let isChecking = true

router.beforeEach(async (to, _, next) => {
  const authStore = useAuthStore()

    if (isChecking) {
    await authStore.checkAuthStatus()
    isChecking = false
  }

  const isLoggedIn = authStore.isAuthenticated

  // kalau butuh login tapi belum login
  if (to.meta.requiresAuth && !isLoggedIn) {
    return next('/')
  }

  // kalau sudah login tapi akses login page
  if (to.meta.guestOnly && isLoggedIn) {
    const profile = await authStore.fetchProfile()

    if (profile.role === 'admin') {
      return next('/admin-dashboard')
    } else {
      return next('/user-dashboard')
    }
  }

  next()
})

export default router