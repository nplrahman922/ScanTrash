import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"
import { useAuthStore } from "./authStore"

// Tipe data TransactionItem yang dikirim backend (kalkulasi sudah dilakukan di Rust)
interface TransactionItem {
  id: string
  name: string          // keterangan atau default "Setoran"/"Penarikan"
  date: string          // ISO timestamptz dari created_at
  nominal: number       // selisih absolut amount - amount_before (dihitung backend)
  transaction_type: "income" | "expense"  // ditentukan backend
}

// Tipe data dari tabel tanggal
interface ScheduleItem {
  id_tanggal?: string
  waktu_buka: string
  lokasi: string
  waktu_tutup: string
  tanggal: string
}

// Tipe data yang dipakai TransactionCard di UI
interface Transaction {
  id: string
  name: string
  date: string
  amount: number
  type: "income" | "expense"
}

export const useUserStore = defineStore("user", {
  state: () => ({
    balance: 0 as number,
    totalTrash: 1,
    schedule: "Besok, 09:00",
    transactions: [] as Transaction[],
    loadingBalance: false,
    loadingHistory: false,
    loadingSchedule: false,
    errorBalance: "" as string,
    errorHistory: "" as string,
    errorSchedule: "" as string,
  }),

  actions: {
    // Ambil saldo terkini dari backend Rust.
    // Wajib menyertakan user_id dari profil yang sedang login.
    async fetchBalance() {
      this.loadingBalance = true
      this.errorBalance = ""
      try {
        const authStore = useAuthStore()
        const userId = authStore.profile?.user_id
        if (!userId) {
          throw new Error("Tidak dapat memuat saldo: profil pengguna belum tersedia.")
        }
        const saldo = await invoke<number>("get_balance_command", { targetUserId: userId })
        this.balance = saldo
      } catch (err: any) {
        this.errorBalance = err as string
        console.error("[userStore] Gagal ambil saldo:", err)
      } finally {
        this.loadingBalance = false
      }
    },

    // Ambil riwayat transaksi dari backend Rust.
    // Wajib menyertakan user_id dari profil yang sedang login.
    // Kalkulasi (nominal, income/expense) sudah dilakukan di backend.
    async fetchHistory() {
      this.loadingHistory = true
      this.errorHistory = ""
      try {
        const authStore = useAuthStore()
        const userId = authStore.profile?.user_id
        if (!userId) {
          throw new Error("Tidak dapat memuat riwayat: profil pengguna belum tersedia.")
        }
        const items = await invoke<TransactionItem[]>("get_savings_history_command", { targetUserId: userId })

        // Hanya format tanggal untuk presentasi (logika bisnis sudah di backend)
        this.transactions = items.map((item) => {
          let dateStr = item.date
          if (dateStr) {
            const d = new Date(dateStr)
            dateStr = `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
          }
          return {
            id: item.id,
            name: item.name,
            date: dateStr || "-",
            amount: item.nominal,
            type: item.transaction_type,
          }
        })
      } catch (err: any) {
        this.errorHistory = err as string
        console.error("[userStore] Gagal ambil riwayat:", err)
      } finally {
        this.loadingHistory = false
      }
    },

    // Ambil jadwal setor dari backend Rust
    async fetchSchedule() {
      this.loadingSchedule = true
      this.errorSchedule = ""
      try {
        const schedules = await invoke<ScheduleItem[]>("get_schedules_command")

        if (schedules && schedules.length > 0) {
          // Ambil jadwal teratas (karena sudah diurutkan asc dari backend)
          const nextSchedule = schedules[0]
          
          // Format tanggal (misal "2026-05-10" -> "10/5/2026")
          let dateStr = nextSchedule.tanggal
          if (dateStr) {
            const d = new Date(dateStr)
            dateStr = `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
          }
          
          // Format waktu (misal "09:00:00" -> "09:00")
          const timeStr = nextSchedule.waktu_buka ? nextSchedule.waktu_buka.substring(0, 5) : ""

          this.schedule = `${dateStr}, ${timeStr}`
        } else {
          this.schedule = "Belum ada jadwal"
        }
      } catch (err: any) {
        this.errorSchedule = err as string
        this.schedule = "Gagal memuat"
        console.error("[userStore] Gagal ambil jadwal:", err)
      } finally {
        this.loadingSchedule = false
      }
    },
  },
})