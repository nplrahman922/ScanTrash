import { defineStore } from "pinia"

export const useTipsStore = defineStore("tips", {
  state: () => ({
    tips: [
      {
        id: 1,
        title: "Pisahkan Sampah Organik",
        description: "Kumpulkan sisa makanan, daun, dan bahan alami untuk dijadikan kompos.",
        icon: "/src/assets/userAset/Tips.svg",
        highlight: true
      },
      {
        id: 2,
        title: "Kelompokkan Sampah Daur Ulang",
        description: "Simpan plastik, kertas, kardus, dan logam di wadah khusus agar mudah didaur ulang.",
        icon: "/src/assets/userAset/Tips2.svg",
        highlight: false
      },
      {
        id: 3,
        title: "Pisahkan Sampah Berbahaya",
        description: "Baterai, lampu, dan obat kadaluarsa harus dipisahkan agar tidak mencemari lingkungan.",
        icon: "/src/assets/userAset/Tips.svg",
        highlight: true
      },
      {
        id: 4,
        title: "Sediakan Tempat untuk Sampah Residu",
        description: "Buang sampah yang tidak bisa didaur ulang (misalnya tisu, popok) ke wadah terpisah.",
        icon: "/src/assets/userAset/Tips2.svg",
        highlight: false
      }
    ]
  })
})