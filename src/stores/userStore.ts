import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

// Tipe data satu baris dari tabel savings (sesuai SavingsRecord di Rust)
interface SavingsRecord {
  id?: string
  user_id?: string
  amount_before?: number  // saldo SEBELUM transaksi
  amount?: number         // saldo SETELAH transaksi
  keterangan?: string     // catatan transaksi
  created_at?: string
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
    // Ambil saldo terkini dari backend Rust
    async fetchBalance() {
      this.loadingBalance = true
      this.errorBalance = ""
      try {
        const saldo = await invoke<number>("get_balance_command", { targetUserId: null })
        this.balance = saldo
      } catch (err: any) {
        this.errorBalance = err as string
        console.error("[userStore] Gagal ambil saldo:", err)
      } finally {
        this.loadingBalance = false
      }
    },

    // Ambil riwayat transaksi dari backend Rust
    async fetchHistory() {
      this.loadingHistory = true
      this.errorHistory = ""
      try {
        const records = await invoke<SavingsRecord[]>("get_savings_history_command", { targetUserId: null })

        // Map SavingsRecord ke format yang dipahami TransactionCard
        this.transactions = records.map((rec) => {
          const before = rec.amount_before ?? 0
          const after  = rec.amount ?? 0
          // Jika saldo naik → income, turun → expense
          const isIncome = after >= before

          // Format tanggal dari ISO timestamptz → "D/M/YYYY"
          let dateStr = "-"
          if (rec.created_at) {
            const d = new Date(rec.created_at)
            dateStr = `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
          }

          // Nominal = selisih absolut antara saldo sebelum dan sesudah
          const nominal = Math.abs(after - before)

          return {
            id: rec.id ?? Math.random().toString(),
            name: rec.keterangan || (isIncome ? "Setoran" : "Penarikan"),
            date: dateStr,
            amount: nominal,
            type: isIncome ? "income" : "expense",
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