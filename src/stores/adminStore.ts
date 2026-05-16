import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"

// ─── Types ───────────────────────────────────────────────────────────────────

export interface AktivitasItem {
  id: string
  username: string
  user_id: string
  date: string
  keterangan: string
  nominal: number
  transaction_type: "income" | "expense"
}

export interface AdminDashboard {
  total_saldo: number
  total_nasabah: number
  aktivitas_terbaru: AktivitasItem[]
}

export interface NasabahItem {
  user_id: string
  username: string
  email: string
  photo_url?: string
  saldo: number
}

export interface Schedule {
  id_tanggal?: string
  tanggal: string       // "YYYY-MM-DD"
  waktu_buka: string    // "HH:MM:SS+08" dari DB
  waktu_tutup: string   // "HH:MM:SS+08" dari DB
  lokasi: string
}

interface ScheduleFormState {
  tanggal: string      // "YYYY-MM-DD" untuk input date
  waktuBuka: string    // "HH:MM" untuk input time
  waktuTutup: string   // "HH:MM" untuk input time
  lokasi: string
}

export interface PricelistItem {
  id?: string
  labels: string
  price: number
  img_url?: string
  created_at?: string
}

interface PricelistFormState {
  labels: string
  price: number
  imageBase64: string        // base64 PNG tanpa prefix; kosong = tidak ganti gambar
  imagePreviewUrl: string    // data URL untuk preview di UI
  currentImgUrl: string      // URL gambar saat ini (untuk delete saat edit)
}

// ─── Store ────────────────────────────────────────────────────────────────────

export const useAdminStore = defineStore("admin", {
  state: () => ({
    // Dashboard
    totalSaldo: 0 as number,
    totalNasabah: 0 as number,
    aktivitasTerbaru: [] as AktivitasItem[],
    loadingDashboard: false,
    errorDashboard: "" as string,

    // Nasabah list
    nasabahList: [] as NasabahItem[],
    searchKeyword: "" as string,
    loadingNasabah: false,
    errorNasabah: "" as string,

    // Form Setoran
    showSetoranModal: false,
    setoranTargetUserId: "" as string,
    setoranTargetUsername: "" as string,
    setoranNominal: 0 as number,
    setoranKeterangan: "" as string,
    loadingSetoran: false,
    errorSetoran: "" as string,
    successSetoran: false,

    // Form Penarikan
    showPenarikanModal: false,
    penarikanTargetUserId: "" as string,
    penarikanTargetUsername: "" as string,
    penarikanNominal: 0 as number,
    penarikanKeterangan: "" as string,
    loadingPenarikan: false,
    errorPenarikan: "" as string,
    successPenarikan: false,

    // Jadwal
    scheduleList: [] as Schedule[],
    loadingSchedule: false,
    errorSchedule: "" as string,
    showScheduleModal: false,
    editingScheduleId: null as string | null,  // null = mode tambah
    scheduleForm: {
      tanggal: "",
      waktuBuka: "",
      waktuTutup: "",
      lokasi: "",
    } as ScheduleFormState,
    loadingScheduleForm: false,
    errorScheduleForm: "" as string,
    deletingScheduleId: null as string | null,  // ID yang sedang dikonfirmasi hapus

    // Pricelist
    pricelistItems: [] as PricelistItem[],
    loadingPricelist: false,
    errorPricelist: "" as string,
    showPricelistModal: false,
    editingPricelistId: null as string | null,  // null = mode tambah
    pricelistForm: {
      labels: "",
      price: 0,
      imageBase64: "",
      imagePreviewUrl: "",
      currentImgUrl: "",
    } as PricelistFormState,
    loadingPricelistForm: false,
    errorPricelistForm: "" as string,
    deletingPricelistId: null as string | null,
  }),

  actions: {
    // ── Fetch data dashboard admin (total saldo, total nasabah, aktivitas terbaru)
    async fetchDashboard() {
      this.loadingDashboard = true
      this.errorDashboard = ""
      try {
        const data = await invoke<AdminDashboard>("get_admin_dashboard_command")
        this.totalSaldo = data.total_saldo
        this.totalNasabah = data.total_nasabah

        // Format tanggal aktivitas untuk presentasi
        this.aktivitasTerbaru = data.aktivitas_terbaru.map((item) => {
          let dateStr = item.date
          if (dateStr) {
            const d = new Date(dateStr)
            dateStr = `${d.getDate()}/${d.getMonth() + 1}/${d.getFullYear()}`
          }
          return { ...item, date: dateStr || "-" }
        })
      } catch (err: any) {
        this.errorDashboard = String(err)
        console.error("[adminStore] Gagal ambil dashboard:", err)
      } finally {
        this.loadingDashboard = false
      }
    },

    // ── Fetch daftar nasabah, dengan opsional keyword pencarian
    async fetchNasabah(keyword: string = "") {
      this.loadingNasabah = true
      this.errorNasabah = ""
      this.searchKeyword = keyword
      try {
        const list = await invoke<NasabahItem[]>("get_nasabah_list_command", {
          keyword,
        })
        this.nasabahList = list
      } catch (err: any) {
        this.errorNasabah = String(err)
        console.error("[adminStore] Gagal ambil daftar nasabah:", err)
      } finally {
        this.loadingNasabah = false
      }
    },

    // ── Pilih nasabah untuk form setoran (dari halaman nasabah mode select)
    selectNasabahForSetoran(userId: string, username: string) {
      this.setoranTargetUserId = userId
      this.setoranTargetUsername = username
    },

    // ── Pilih nasabah untuk form penarikan (dari halaman nasabah mode select)
    selectNasabahForPenarikan(userId: string, username: string) {
      this.penarikanTargetUserId = userId
      this.penarikanTargetUsername = username
    },

    // ── Reset form setoran
    resetSetoranForm() {
      this.setoranTargetUserId = ""
      this.setoranTargetUsername = ""
      this.setoranNominal = 0
      this.setoranKeterangan = ""
      this.errorSetoran = ""
      this.successSetoran = false
    },

    // ── Reset form penarikan
    resetPenarikanForm() {
      this.penarikanTargetUserId = ""
      this.penarikanTargetUsername = ""
      this.penarikanNominal = 0
      this.penarikanKeterangan = ""
      this.errorPenarikan = ""
      this.successPenarikan = false
    },

    // ── Kirim setoran ke backend
    async createSetoran() {
      this.loadingSetoran = true
      this.errorSetoran = ""
      this.successSetoran = false
      try {
        await invoke("create_setoran_command", {
          targetUserId: this.setoranTargetUserId,
          nominal: this.setoranNominal,
          keterangan: this.setoranKeterangan,
        })
        this.successSetoran = true
        await this.fetchDashboard()
      } catch (err: any) {
        this.errorSetoran = String(err)
        console.error("[adminStore] Gagal setoran:", err)
      } finally {
        this.loadingSetoran = false
      }
    },

    // ── Kirim penarikan ke backend
    async createPenarikan() {
      this.loadingPenarikan = true
      this.errorPenarikan = ""
      this.successPenarikan = false
      try {
        await invoke("create_penarikan_command", {
          targetUserId: this.penarikanTargetUserId,
          nominal: this.penarikanNominal,
          keterangan: this.penarikanKeterangan,
        })
        this.successPenarikan = true
        await this.fetchDashboard()
      } catch (err: any) {
        this.errorPenarikan = String(err)
        console.error("[adminStore] Gagal penarikan:", err)
      } finally {
        this.loadingPenarikan = false
      }
    },

    // ── Jadwal: ambil semua jadwal
    async fetchSchedules() {
      this.loadingSchedule = true
      this.errorSchedule = ""
      try {
        const list = await invoke<Schedule[]>("get_schedules_command")
        this.scheduleList = list
      } catch (err: any) {
        this.errorSchedule = String(err)
        console.error("[adminStore] Gagal ambil jadwal:", err)
      } finally {
        this.loadingSchedule = false
      }
    },

    // ── Jadwal: buka modal tambah
    openAddScheduleModal() {
      this.editingScheduleId = null
      this.scheduleForm = { tanggal: "", waktuBuka: "", waktuTutup: "", lokasi: "" }
      this.errorScheduleForm = ""
      this.showScheduleModal = true
    },

    // ── Jadwal: buka modal edit dengan data pre-filled
    openEditScheduleModal(schedule: Schedule) {
      this.editingScheduleId = schedule.id_tanggal ?? null
      this.scheduleForm = {
        tanggal: schedule.tanggal,
        // Ambil "HH:MM" dari format "HH:MM:SS+08"
        waktuBuka: schedule.waktu_buka.slice(0, 5),
        waktuTutup: schedule.waktu_tutup.slice(0, 5),
        lokasi: schedule.lokasi,
      }
      this.errorScheduleForm = ""
      this.showScheduleModal = true
    },

    // ── Jadwal: tutup modal
    closeScheduleModal() {
      this.showScheduleModal = false
      this.editingScheduleId = null
      this.errorScheduleForm = ""
    },

    // ── Jadwal: submit (tambah atau edit tergantung mode)
    async submitSchedule() {
      const f = this.scheduleForm
      if (!f.tanggal || !f.waktuBuka || !f.waktuTutup || !f.lokasi) {
        this.errorScheduleForm = "Semua field wajib diisi."
        return
      }
      this.loadingScheduleForm = true
      this.errorScheduleForm = ""
      try {
        if (this.editingScheduleId) {
          // Mode edit
          await invoke("update_schedule_command", {
            idTanggal: this.editingScheduleId,
            tanggal: f.tanggal,
            waktuBuka: f.waktuBuka,
            waktuTutup: f.waktuTutup,
            lokasi: f.lokasi,
          })
        } else {
          // Mode tambah
          await invoke("create_schedule_command", {
            tanggal: f.tanggal,
            waktuBuka: f.waktuBuka,
            waktuTutup: f.waktuTutup,
            lokasi: f.lokasi,
          })
        }
        this.showScheduleModal = false
        await this.fetchSchedules()  // Refresh list
      } catch (err: any) {
        this.errorScheduleForm = String(err)
        console.error("[adminStore] Gagal simpan jadwal:", err)
      } finally {
        this.loadingScheduleForm = false
      }
    },

    // ── Jadwal: hapus
    async deleteSchedule(idTanggal: string) {
      try {
        await invoke("delete_schedule_command", { idTanggal })
        this.deletingScheduleId = null
        await this.fetchSchedules()  // Refresh list
      } catch (err: any) {
        console.error("[adminStore] Gagal hapus jadwal:", err)
        throw err  // Teruskan ke component agar bisa tampilkan error
      }
    },

    // ── Pricelist: ambil semua item
    async fetchPricelist() {
      this.loadingPricelist = true
      this.errorPricelist = ""
      try {
        const list = await invoke<PricelistItem[]>("get_pricelist_command")
        this.pricelistItems = list
      } catch (err: any) {
        this.errorPricelist = String(err)
        console.error("[adminStore] Gagal ambil pricelist:", err)
      } finally {
        this.loadingPricelist = false
      }
    },

    // ── Pricelist: buka modal tambah
    openAddPricelistModal() {
      this.editingPricelistId = null
      this.pricelistForm = { labels: "", price: 0, imageBase64: "", imagePreviewUrl: "", currentImgUrl: "" }
      this.errorPricelistForm = ""
      this.showPricelistModal = true
    },

    // ── Pricelist: buka modal edit dengan data pre-filled
    openEditPricelistModal(item: PricelistItem) {
      this.editingPricelistId = item.id ?? null
      this.pricelistForm = {
        labels: item.labels,
        price: item.price,
        imageBase64: "",                        // Kosong = tidak ganti gambar
        imagePreviewUrl: item.img_url ?? "",    // Preview gambar saat ini
        currentImgUrl: item.img_url ?? "",      // Simpan URL lama untuk delete nanti
      }
      this.errorPricelistForm = ""
      this.showPricelistModal = true
    },

    // ── Pricelist: tutup modal
    closePricelistModal() {
      this.showPricelistModal = false
      this.editingPricelistId = null
      this.errorPricelistForm = ""
    },

    // ── Pricelist: submit (tambah atau edit)
    async submitPricelist() {
      const f = this.pricelistForm
      if (!f.labels.trim()) {
        this.errorPricelistForm = "Nama sampah tidak boleh kosong."
        return
      }
      if (f.price <= 0) {
        this.errorPricelistForm = "Harga harus lebih dari Rp0."
        return
      }
      // Untuk mode tambah, gambar wajib
      if (!this.editingPricelistId && !f.imageBase64) {
        this.errorPricelistForm = "Pilih gambar untuk item baru."
        return
      }

      this.loadingPricelistForm = true
      this.errorPricelistForm = ""
      try {
        if (this.editingPricelistId) {
          await invoke("update_pricelist_command", {
            id: this.editingPricelistId,
            labels: f.labels,
            price: f.price,
            imageBase64: f.imageBase64,
            currentImgUrl: f.currentImgUrl,
          })
        } else {
          await invoke("create_pricelist_command", {
            labels: f.labels,
            price: f.price,
            imageBase64: f.imageBase64,
          })
        }
        this.showPricelistModal = false
        await this.fetchPricelist()
      } catch (err: any) {
        this.errorPricelistForm = String(err)
        console.error("[adminStore] Gagal simpan pricelist:", err)
      } finally {
        this.loadingPricelistForm = false
      }
    },

    // ── Pricelist: hapus item
    async deletePricelist(id: string, imgUrl: string) {
      try {
        await invoke("delete_pricelist_command", { id, imgUrl })
        this.deletingPricelistId = null
        await this.fetchPricelist()
      } catch (err: any) {
        console.error("[adminStore] Gagal hapus pricelist:", err)
        throw err
      }
    },
  },
})
