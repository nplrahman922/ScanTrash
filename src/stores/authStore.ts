import { defineStore } from "pinia"
import { auth } from "../firebase"
import { invoke } from "@tauri-apps/api/core"
import { signInWithPopup, GoogleAuthProvider, signOut } from "firebase/auth"
import type { LoginCredentials, LoginResponse } from "../types/auth"

interface AuthState {
  user: any | null
  token: string | null
  loading: boolean
  error: string | null
}

export const useAuthStore = defineStore("auth", {
  state: (): AuthState => ({
    user: null,
    token: localStorage.getItem("token") || null,
    loading: false,
    error: null
  }),

  getters: {
    isAuthenticated: (state): boolean => !!state.token
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

        if (this.token) {
          localStorage.setItem("token", this.token)
        }

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
        const provider = new GoogleAuthProvider()
        const result = await signInWithPopup(auth, provider)
        this.setUser(result.user)
      } catch (error: any) {
        this.setError(error.message)
      } finally {
        this.loading = false
      }
    },

    async logout() {
      try {
        await signOut(auth)
        this.user = null
        this.token = null
        localStorage.removeItem("token")
        this.error = null
      } catch (error: any) {
        this.setError(error.message)
      }
    },

    setUser(user: any) {
      this.user = user
      this.error = null
    },

    setError(error: string) {
      this.error = error
      this.loading = false
    },
  },
})