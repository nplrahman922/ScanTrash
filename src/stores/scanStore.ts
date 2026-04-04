import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

interface ScanResult {
  name: string
  status: string
  price: number
  details: {
    material: string
    condition: string
    cleanliness: string
  }
}

export const useScanStore = defineStore("scan", {
  state: () => ({
    loading: false,
    result: null as ScanResult | null,
    error: null as string | null
  }),

  actions: {
    async scanTrash(image: string) {
      this.loading = true
      this.error = null

      try {
        const res = await invoke<ScanResult>("scan_trash", { image })
        this.result = res
      } catch (err: any) {
        this.error = err?.message || "Scan gagal"
      } finally {
        this.loading = false
      }
    },

    resetScan() {
      this.result = null
      this.error = null
    }
  }
})