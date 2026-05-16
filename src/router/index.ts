import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '../stores/authStore'
import MainLayout from '../layout/MainLayout.vue'
import LoginView from '../views/LoginView.vue'
import UserDashboard from '../views/UserDashboard.vue'
import WalletView from '../views/WalletView.vue'
import TrashTypeView from '../views/TrashTypeView.vue'
import TipsView from '../views/TipsView.vue'
import AdminDashboard from '../views/AdminDashboard.vue'
import AdminNasabah from '../views/AdminNasabah.vue'
import AdminJadwal from '../views/AdminJadwal.vue'
import AdminKatalog from '../views/AdminKatalog.vue'
import ScanView from '../views/ScanView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/login',
    name: 'Login',
    component: LoginView,
    meta: { guestOnly: true }
  },
  {
    path: '/scan',
    name: 'Scan',
    component: ScanView,
    meta: { requiresAuth: true }
  },
  {
    path: '/',
    component: MainLayout,
    children: [
      {
        path: 'user-dashboard',
        name: 'UserDashboard',
        component: UserDashboard,
        meta: { requiresAuth: true }
      },
      {
        path: 'wallet',
        name: 'Wallet',
        component: WalletView,
        meta: { requiresAuth: true }
      },
      {
        path: 'jenis-sampah',
        name: 'TrashTypeView',
        component: TrashTypeView,
        meta: { requiresAuth: true }
      },
      {
        path: 'tips-memilah-sampah',
        name: 'TipsMemilahSampah',
        component: TipsView,
        meta: { requiresAuth: true }
      },
      {
        path: 'admin-dashboard',
        name: 'AdminDashboard',
        component: AdminDashboard,
        meta: { requiresAuth: true }
      },
      {
        path: 'admin-nasabah',
        name: 'AdminNasabah',
        component: AdminNasabah,
        meta: { requiresAuth: true }
      },
      {
        path: 'admin-jadwal',
        name: 'AdminJadwal',
        component: AdminJadwal,
        meta: { requiresAuth: true }
      },
      {
        path: 'admin-katalog',
        name: 'AdminKatalog',
        component: AdminKatalog,
        meta: { requiresAuth: true }
      },
    ]
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

  // 🔥 FIX BUG LAYAR KOSONG: Tangani rute root ('/')
  if (to.path === '/') {
    if (!isLoggedIn) {
      return next('/login')
    } else {
      const profile = await authStore.fetchProfile()
      if (profile.role === 'admin') {
        return next({ name: 'AdminDashboard' })
      } else {
        return next({ name: 'UserDashboard' })
      }
    }
  }

  // kalau butuh login tapi belum login
  if (to.meta.requiresAuth && !isLoggedIn) {
    return next('/login')
  }

  // kalau sudah login tapi akses login page
  if (to.meta.guestOnly && isLoggedIn) {
    const profile = await authStore.fetchProfile()

    if (profile.role === 'admin') {
      return next({ name: 'AdminDashboard' })
    } else {
      return next({ name: 'UserDashboard' })
    }
  }

  next()
})

export default router