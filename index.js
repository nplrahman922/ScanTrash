require('dotenv').config();
const express = require('express');
const { createClient } = require('@supabase/supabase-js');

const app = express();
const port = 3000;

const supabase = createClient(process.env.SUPABASE_URL, process.env.SUPABASE_ANON_KEY);

// 1. Halaman Awal
app.get('/', (req, res) => {
  res.send(`
    <h2>Test Login Supabase</h2>
    <a href="/auth/google" style="padding: 10px; background: #4285F4; color: white; text-decoration: none; border-radius: 5px;">Login dengan Google</a>
  `);
});

// 2. Redirect ke Google
app.get('/auth/google', async (req, res) => {
  const { data, error } = await supabase.auth.signInWithOAuth({
    provider: 'google',
    options: { redirectTo: `http://localhost:${port}/auth/callback` },
  });
  
  if (error) return res.status(400).send(error.message);
  res.redirect(data.url);
});

// 3. Callback Penangkap Token
app.get('/auth/callback', (req, res) => {
  res.send(`
    <h3>Sedang memproses login...</h3>
    <script>
      const hash = window.location.hash;
      if (hash) {
        const params = new URLSearchParams(hash.substring(1));
        const token = params.get('access_token');
        
        if (token) {
          fetch('/api/get-role', {
            headers: { 'Authorization': 'Bearer ' + token }
          })
          .then(res => res.json())
          .then(data => {
            document.body.innerHTML = '<h2>Hasil Data:</h2><pre>' + JSON.stringify(data, null, 2) + '</pre>';
          })
          .catch(err => document.body.innerHTML = 'Error: ' + err);
        }
      } else {
        document.body.innerHTML = 'Gagal menangkap token dari Google.';
      }
    </script>
  `);
});

// 4. API Pengambil Data (Target Akhir)
app.get('/api/get-role', async (req, res) => {
  const authHeader = req.headers.authorization;
  if (!authHeader) return res.status(401).json({ error: "Tidak ada token" });
  
  const token = authHeader.split(' ')[1];
  
  // Verifikasi token
  const { data: { user }, error: authError } = await supabase.auth.getUser(token);
  if (authError || !user) return res.status(401).json({ error: "Token tidak valid" });

  // --- PERBAIKAN UTAMA DI SINI ---
  // Buat client Supabase "sekali pakai" khusus untuk request ini dengan membawa token JWT user
  // Ini memastikan RLS mengizinkan select ke tabel profiles
  const userSupabase = createClient(
    process.env.SUPABASE_URL, 
    process.env.SUPABASE_ANON_KEY,
    { global: { headers: { Authorization: `Bearer ${token}` } } }
  );

  // Ambil profil
  const { data: profile, error: profileError } = await userSupabase
    .from('profiles')
    .select('*')
    .eq('user_id', user.id)
    .single();

  if (profileError) {
    // Menangkap error jika profil tetap tidak ditemukan (biasanya karena Trigger SQL belum dieksekusi)
    return res.status(404).json({ 
      error: "Data profil kosong.", 
      detail: profileError.message,
      solusi: "Cek tabel 'profiles' di Supabase, apakah datanya ada? Jika kosong, jalankan ulang script Trigger SQL."
    });
  }

  res.json({
    status: "Sukses Login!",
    user_id: user.id,
    role_pengguna: profile.role,
    data_lengkap: profile
  });
});

app.listen(port, () => {
  console.log('====================================');
  console.log(`🚀 Server jalan di http://localhost:${port}`);
  console.log('====================================');
});