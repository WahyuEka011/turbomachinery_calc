# Turbomachinery Calculator — Rust + Qt5 Integration
## TURBOMACHINERY_CALC by Kelompok 2: Project ini berisi Pemrogramman Perhitungan Metode Lookup Table dan Deret Taylor, Machine Learning SVM (Support Vector Machine) dan kNN, lalu ANN

### Deskripsi

Proyek ini adalah integrasi Rust dan Qt untuk mensimulasikan prediksi kondisi turbin menggunakan Artificial Neural Network (ANN).
Rust bertugas melakukan komputasi backend, sedangkan Qt5 bertugas sebagai antarmuka pengguna (GUI).
Seluruh proyek dapat langsung dicoba melalui GitHub Codespaces!

### Struktur Project ini :
turbomachinery_calc/
├── rust_backend/      # Backend Rust (ANN, prediksi, dsb.)
│   ├── src/
│   ├── Cargo.toml
│   └── target/release/libturbomachinery_calc.so (library Rust)
├── qt_frontend/       # Frontend Qt5 (GUI input/output)
│   ├── mainwindow.ui
│   ├── annwindow.ui
│   ├── homepagewindow.ui
│   ├── mainwindow.cpp
│   ├── annwindow.cpp
│   ├── homepagewindow.cpp
│   └── CMakeLists.txt
└── README.md          # Penjelasan project

### 🚀 Cara Menjalankan Proyek (Via GitHub Codespaces)
1. Buka GitHub repository ini.
2. Klik tombol <> Code → pilih Open in Codespaces → buat Codespace baru.
3. Setelah Codespace terbuka:
   // Pastikan posisi di direktori project utama
      cd turbomachinery_calc
4. Build library Rust (librust_backend.so):
   cd rust_backend
   cargo build --release
5. Build Qt Frontend:
6. cd ../qt_frontend
   mkdir build
   cd build
   cmake ..
   make
7. Jalankan aplikasi:
   ./QtRustIntegration
8. 🎉 Aplikasi GUI Qt akan tampil!

    Masukkan input data (Electricity, RPM, Steam Pressure, Vibration).
    Klik Predict untuk mendapatkan hasil prediksi kondisi turbin.
    Prediksi juga menampilkan grafik!

### 📚 Penjelasan Teknis
Rust Backend:
    >) Berisi logika ANN.
    >) Data inputan berupa fitur-fitur turbin.
    >) Output berupa string hasil prediksi ("Normal", "Maintenance", atau "Fail").

Qt Frontend:
    >) Dibangun menggunakan Qt5 Widgets.
    >) Mengambil inputan dari user.
    >) Memanggil fungsi dari library Rust via FFI (extern "C").
    >) Menampilkan hasil prediksi di tampilan GUI.

Integrasi Rust dan Qt:
    >) Rust meng-compile kode menjadi .so (shared object library).
    >) Qt menggunakan CMakeLists.txt untuk link ke libturbomachinery_calc.so.
    >) Fungsi Rust diekspor ke Qt menggunakan deklarasi extern "C".

### ⚙️ Dependensi
>) Rust (stable version)
>) Cargo (built-in Rust package manager)
>) Qt5 (qtbase5-dev, qt5-qmake, qttools5-dev-tools)
>) CMake (versi 3.5 ke atas)

Semua ini sudah tersedia otomatis di GitHub Codespaces, jadi kamu tinggal jalankan!

### ✨ Catatan
>) Setiap kali kamu mengubah kode di rust_backend, jangan lupa build ulang cargo build --release supaya file .so terupdate.
>) Untuk mendebug error saat build Qt, lihat bagian error make di terminal Codespaces.
>) Jika tampilan GUI tidak muncul di Codespaces GUI secara default, jalankan Codespaces > Ports > Forward Port di menu.
