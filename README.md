# Turbomachinery Calculator — Rust + Qt5 Integration
## TURBOMACHINERY_CALC by Kelompok 2: Project ini berisi Pemrogramman Perhitungan Metode Lookup Table dan Deret Taylor, Machine Learning SVM (Support Vector Machine) dan kNN, lalu ANN

### Deskripsi

Proyek ini adalah integrasi Rust dan Qt untuk mensimulasikan prediksi kondisi turbin menggunakan Artificial Neural Network (ANN).
Rust bertugas melakukan komputasi backend, sedangkan Qt5 bertugas sebagai antarmuka pengguna (GUI).
Seluruh proyek dapat langsung dicoba melalui GitHub Codespaces!

Struktur Project ini :
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

🚀 Cara Menjalankan Proyek (Via GitHub Codespaces)
