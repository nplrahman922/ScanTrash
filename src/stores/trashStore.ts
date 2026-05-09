import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

// Tipe data dari backend (sesuai struct Pricelist di Rust)
interface PricelistItem {
  id?: string
  labels: string
  price: number
  img_url?: string
  created_at?: string
}

// Tipe data yang dipakai TrashTypeCard di UI
interface TrashType {
  id: string
  name: string
  price: number
  image: string
}

export const useTrashStore = defineStore("trash", {
  state: () => ({
    trashTypes: [] as TrashType[],
    loading: false,
    error: "" as string,
  }),

  actions: {
    async fetchTrashTypes() {
      this.loading = true
      this.error = ""
      try {
        const items = await invoke<PricelistItem[]>("get_pricelist_command")

        // Map PricelistItem → TrashType (format yang dipahami TrashTypeCard)
        this.trashTypes = items.map((item) => ({
          id: item.id ?? Math.random().toString(),
          name: item.labels,
          price: item.price,
          image: item.img_url ?? "",
        }))
      } catch (err: any) {
        this.error = err as string
        console.error("[trashStore] Gagal ambil pricelist:", err)
      } finally {
        this.loading = false
      }
    },
  },
})