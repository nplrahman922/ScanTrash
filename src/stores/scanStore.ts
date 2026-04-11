import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

// 👇 1. INTERFACE DIUPDATE JADI FLAT (Sesuai Database & Rust)
export interface ScanResult {
  trash_type: string
  label_id: string
  material_info: string
  kondisi: string
  // kebersihan: string
  estimasi_harga: number
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
        // 👇 2. Parameter dikirim sebagai { image } sesuai kesepakatan Rust terakhir
        const res = await invoke<ScanResult>("scan_trash", { image })
        this.result = res
      } catch (err: any) {
        // Log untuk mempermudah nyari bug kalau ada error dari Rust
        console.error("Error dari Rust/Tauri:", err); 
        
        // Tangkap error dengan aman
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