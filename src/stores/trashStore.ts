import { defineStore } from "pinia"

export const useTrashStore = defineStore("trash", {
  state: () => ({
    trashTypes: [
      {
        id: 1,
        name: "Plastik (PET)",
        price: 4000,
        image: "/src/assets/sampah/plastik.png" // opsional
      },
      {
        id: 2,
        name: "Kertas/Kardus",
        price: 2500,
        image: "" // ❌ belum ada gambar
      },
      {
        id: 3,
        name: "Logam/Besi",
        price: 8000,
        image: ""
      },
      {
        id: 4,
        name: "Kaca/Beling",
        price: 1500,
        image: ""
      }
    ]
  }),

//   // 🔥 nanti tinggal pakai ini kalau dari backend
//   actions: {
//     async fetchTrashTypes() {
//       // contoh:
//       // const res = await axios.get('/trash-types')
//       // this.trashTypes = res.data
//     }
//   }
})