import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

export interface ScanResult {
  // Tidak perlu id, created_at, update_at karena itu urusan DB nanti
  trash_type: string;
  label_id: string;
  material_info: string;
  kondisi: string;
  kebersihan: string;
  estimasi_harga: number;
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
        // 👇 TAMBAHKAN LOG INI BIAR KITA TAHU ERROR ASLINYA
        console.error("Error dari Rust/Tauri:", err); 
        
        // 👇 UBAH CARA TANGKAP ERRORNYA
        if (typeof err === 'string') {
            this.error = err; // Kalau error dari Rust (String)
        } else {
            this.error = err?.message || "Scan gagal"; // Kalau error dari sistem JS
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