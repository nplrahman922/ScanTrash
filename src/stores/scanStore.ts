import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

export interface ScanResult {
  trash_type: string
  label_id: string
  material_info: string
  kondisi: string
  kebersihan: string // 👈 Dikembalikan
  estimasi_harga: number
}

export const useScanStore = defineStore("scan", {
  state: () => ({
    loading: false,
    result: null as ScanResult[] | null, // 👈 SEKARANG JADI ARRAY
    error: null as string | null
  }),

  actions: {
    async scanTrash(image: string) {
      this.loading = true
      this.error = null

      try {
        // 👇 Tangkap Array dari Rust
        const res = await invoke<ScanResult[]>("scan_trash", { image })
        this.result = res
      } catch (err: any) {
        console.error("Error dari Rust/Tauri:", err); 
        if (typeof err === 'string') {
            this.error = err; 
        } else {
            this.error = err?.message || "Scan gagal"; 
        }
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