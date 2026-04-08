# Program Bot Test

## keluh - Kesah dan klarifikasi pengembang

oke back again ini gw EnBee programer pemula yang sedang belajar ngoding, jadi kalo ada salah salah kata atau salah dalam penulisan kode mohon di maklumi ya, gw juga masih belajar hehe :v .

setelah sedikit meneliti dan mencoba beberap methode , gw memutuskan menggunakan prompted base dengan model ai LLM gratis (gw kanker :v) dengan limit tertentu , alasan utamanya karna bot ini lebih punya otak , memiliki kemampuan perkiraan berat , harga , dan keterangan , serta kemampuan matematis yang mumpuni dan cukup (walau masih sering ngaco, tergantung kondisi). dan karna jika menggunkan metode deep learning pendekatan yang dapat dilakukan akan sangat lebih sulit , ribet dan tidak dapat menghasilkan output yang bervariasi jika kurang pengalaman dalam melakukan training data( gomen guys... ).

## Subbab 1 : Overview Tech

Tech yang digunakan :

- Tauri = TauriV2
- Vue = Vue3
- TypeScript = TypeScript
- Tailwindcss = Tailwindcss
- Rust = Rust
- Hugging Face = Hugging Face
- Qwen = Qwen3-VL-235B-A22B-Instruct

Secara catatan testing dan penggunaan dalam pengembangan bot yang ada saat ini :

- kemampuan akurasi deteksi bot saat ini mencapai 80% - 90% tergantung pada kualitas gambar dan pencahayaan
- memiliki potensi err dalam memuat gambar ( hasil testing dalam pengetesan 100 gambar berbeda)
- batas dari Hugging Face Token free memiliki rasio limit sekitar 40 - 50 img per 0.10$ (tergantung pada ukuran gambar) dalam free limit yang akan di reset per awal bulan
- Payload saat ini menggunakan 1024 sebagai base pemikiran dalam token(walau agak boros, jika terlalu boros kamu dapat menggantinya menjadi 500)
- model ai yang pernah di test dan digunakan adalah Qwen/Qwen3-VL-235B-A22B-Instruct , cohere/aya-vision-32b , dan Qwen/Qwen2.5-VL-30B-Instruct ( disimpulkan deteksi secara umum, jika hanya gambar cohere sedikit lebih unggul , akan tetapi Qwen lebih unggul dalam hal pemahaman konteks dan format output ).

# Implementasi ke production

dalam implementasi ke production , gw akan menghimbau kalian atas beberapa hal berupa yang perlu dikerjakan , diperbaiki , dan pantangan dalam project ini , beberapa hal berikut adalah :

1. Dikerjakan :

- kamu boleh menggunakan kode ( menyalin beberapa methode di api.rs) tapi tetap harus konsisten dengan metode penulisan kode (env ke -> rust) jika menggunakan lib pengaman tambahan seperti stronghold , store , dll , tetap harus konsisten dengan metode penulisan kode yang telah dibuat dengan bag login.
- gunakan prinsip 1 function 1 kerjaan (1 function 1 job) , tidak boleh ada fungsi yang memiliki double job atau mendominasi , jadi jika kamu menemukan kode anomali di project ini maka wajib di refactor!!
- gw telah menambahkan fungsi println di beberapa bagian code , jadi kamu dapat memantau alur kerja bot melalui terminal. Hapus jika sudah dalam kondisi final
- tetap terapkan konsep logging walau disini tidak ada penerapannya ( tugas lu bukan tugas gw tu mah :v)

2. saran perbaikan ke production :

- agar dapat dimplementasikan secara fleksible , maka file ai_config.toml dapat kamu pecah dan masukkan beberapa nilai ke dalam database yang sudah dibuat (terutama prompt(bag daftar harga dan contekan , karna sifatnya fluktuatif jadi harus bisa diganti sewaktu waktu ) )
- kamu dapat menyesuaikan kembalian / nilai variabel dalam fungsi - fungsi yang telah dibuat menjadi nama yang berbeda tergantung sesuai kebutuhan yang ada
- kamu dapat menambahkan beberapa fitur tambahan yang menurutmu perlu ditambahkan (selama tidak melanggar aturan yang telah dibuat)

3. Pantangan :

- jangan pernah mengubah metode penulisan kode (env ke -> rust) jika menggunakan lib pengaman tambahan seperti stronghold , store , dll , tetap harus konsisten dengan metode penulisan kode yang telah dibuat dengan bag login.
- jangan pernah mengubah prinsip 1 function 1 kerjaan (1 function 1 job) , tidak boleh ada fungsi yang memiliki double job atau mendominasi , jadi jika kamu menemukan kode anomali di project ini maka wajib di refactor!!
- jangan pernah mengubah( 1 kata pun) dalam file @ai_config.toml , kecuali kamu menggantinya ke format file lainnya (Json,Txt,Database,dll) dan mengikuti aturan yang telah dibuat (berdasarkan pengalaman dalam percobaan , mengubah 1 kata dapat membuat hasil melenceng jauh , kamu dapat mencobanya kalau gk percaya T_T)

## Penutup

segitu aja dari gw , semoga sukses dengan project ini :)
