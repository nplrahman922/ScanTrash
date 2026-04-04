import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

interface ScanResult {
  name: string
  type: string
  price: number
}

export const useScanStore = defineStore("scan", {
  state: () => ({
    loading: false,
    result: null as ScanResult | null,
    error: null as string | null
  }),

  actions: {
    async scanTrash() {
      this.loading = true
      this.error = null

      try {
        const data = await invoke<ScanResult>("scan_trash_command")
        this.result = data
      } catch (err: any) {
        this.error = err.message
      } finally {
        this.loading = false
      }
      // async scanTrash(image: string) {
      //   this.loading = true

      //   try {
      //     const res = await invoke("scan_trash", { image })
      //     this.result = res
      //   } catch (err) {
      //     this.error = err
      //   } finally {
      //     this.loading = false
      //   }
      // }
    }
  }
})