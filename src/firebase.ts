import { initializeApp } from 'firebase/app'
import { getAuth } from 'firebase/auth'

// Konfigurasi Firebase Anda (ganti dengan nilai dari Firebase Console)
const firebaseConfig = {
  apiKey: "your-api-key",
  authDomain: "your-project.firebaseapp.com",
  projectId: "your-project-id",
  storageBucket: "your-project.appspot.com",
  messagingSenderId: "your-sender-id",
  appId: "your-app-id"
}

// Inisialisasi Firebase
const app = initializeApp(firebaseConfig)

// Ekspor auth untuk digunakan di komponen lain
export const auth = getAuth(app)
export default app