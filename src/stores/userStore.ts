// testing pake hardcoded data dulu, nanti tinggal ganti ke API call
import { defineStore } from "pinia"

export const useUserStore = defineStore("user", {
  state: () => ({
    balance: 12000,
    totalTrash: 1,
    schedule: "Besok, 09:00",

    transactions: [
      {
        id: 1,
        name: "Botol Plastik",
        date: "24/2/2026",
        amount: 4000,
        type: "income"
      },
      {
        id: 2,
        name: "Botol Plastik",
        date: "24/2/2026",
        amount: 4000,
        type: "income"
      },
      {
        id: 3,
        name: "Botol Plastik",
        date: "24/2/2026",
        amount: 4000,
        type: "expense"
      }
    ]
  })
})


// import { defineStore } from "pinia"
// import axios from "axios"

// export const useUserStore = defineStore("user", {
//   state: () => ({
//     balance: 0,
//     totalTrash: 0,
//     schedule: "",
//     transactions: [],
//     loading: false
//   }),

//   actions: {
//     async fetchDashboard() {
//       this.loading = true
//       try {
//         const res = await axios.get("http://localhost:3000/dashboard")

//         this.balance = res.data.balance
//         this.totalTrash = res.data.totalTrash
//         this.schedule = res.data.schedule
//         this.transactions = res.data.transactions

//       } catch (error) {
//         console.error("Gagal ambil data:", error)
//       } finally {
//         this.loading = false
//       }
//     }
//   }
// })