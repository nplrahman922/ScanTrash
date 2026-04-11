export interface Profile {
  id: number
  username: string
  email: string
  photo_url?: string
  role: 'admin' | 'user'
}

export interface User {
  id: number
  name: string
  email: string
}

export interface LoginResponse {
  token: string
  user: User
}

export interface LoginCredentials {
  email: string
  password: string
}