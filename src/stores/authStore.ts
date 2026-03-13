import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"
import type { LoginCredentials, LoginResponse } from "../types/auth"

interface AuthState {
  user: any | null
  token: string | null  // JANGAN DIGUNAKAN - backend handle token storage
  loading: boolean
  error: string | null
  profile: any | null
}

export const useAuthStore = defineStore("auth", {
  state: (): AuthState => ({
    user: null,
    token: null, // Jangan store token di frontend
    loading: false,
    error: null,
    profile: null
  }),

  getters: {
    isAuthenticated: (state): boolean => !!state.profile, // Gunakan profile sebagai indikator auth
    userProfile: (state) => state.profile
  },

  actions: {
    async login(credentials: LoginCredentials) {
      this.loading = true
      this.error = null
      try {
        // Panggil Tauri command 'login_user'
        const response = await invoke<LoginResponse>("login_user", {
          email: credentials.email,
          password: credentials.password
        })

        this.token = response.token
        this.user = response.user

        // JANGAN STORE TOKEN DI LOCALSTORAGE - backend handle
        // if (this.token) {
        //   localStorage.setItem("token", this.token)
        // }

      } catch (error) {
        this.error = error instanceof Error ? error.message : "Login gagal"
        console.error("Login gagal", error)
        throw error
      } finally {
        this.loading = false
      }
    },

    async googleLogin() {
      this.loading = true
      this.error = null
      try {
        // Gunakan Tauri invoke untuk Google OAuth
        const authUrl = await invoke('get_google_auth_url_command')
        // Buka URL di browser
        const { openUrl } = await import('@tauri-apps/plugin-opener')
        if (typeof authUrl === "string") {
          await openUrl(authUrl)
        } else {
          throw new Error("Google auth URL is not a string")
        }
        // Tunggu event login-success dari backend
      } catch (error: any) {
        this.setError(error.message)
      } finally {
        this.loading = false
      }
    },

    async logout() {
      try {
        await invoke('logout_command')
        this.user = null
        this.token = null
        // JANGAN REMOVE DARI LOCALSTORAGE - backend handle
        // localStorage.removeItem("token")
        this.error = null
        this.profile = null
      } catch (error: any) {
        this.setError(error.message)
      }
    },

    async checkAuthStatus() {
      try {
        const isLoggedIn = await invoke('check_auth_status_command')
        if (isLoggedIn) {
          await this.fetchProfile()
        }
        return isLoggedIn
      } catch (error) {
        this.setError(error instanceof Error ? error.message : 'Check auth failed')
        return false
      }
    },

    async fetchProfile() {
      try {
        const profile = await invoke('get_profile_command')
        this.setUser(profile)
        return profile
      } catch (error) {
        this.setError(error instanceof Error ? error.message : 'Fetch profile failed')
        throw error
      }
    },

    setUser(profile: any) {
      this.profile = profile
      this.error = null
    },

    setError(error: string) {
      this.error = error
      this.loading = false
    }
  },
})