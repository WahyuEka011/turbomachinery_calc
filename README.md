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
2. Klik tombol <> Code → pilih Open in Codespaces → pilih "CODESPACE_TURBOMACHINERY"
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
1. Rust Backend:
    1. Berisi logika ANN.
    2. Data inputan berupa fitur-fitur turbin.
    3. Output berupa string hasil prediksi ("Normal", "Maintenance", atau "Fail").

2. Qt Frontend:
    1. Dibangun menggunakan Qt5 Widgets.
    2. Mengambil inputan dari user.
    3. Memanggil fungsi dari library Rust via FFI (extern "C").
    4. Menampilkan hasil prediksi di tampilan GUI.

3. Integrasi Rust dan Qt:
    1. Rust meng-compile kode menjadi .so (shared object library).
    2. Qt menggunakan CMakeLists.txt untuk link ke libturbomachinery_calc.so.
    3. Fungsi Rust diekspor ke Qt menggunakan deklarasi extern "C".

### ⚙️ Dependensi
1. Rust (stable version)
2. Cargo (built-in Rust package manager)
3. Qt5 (qtbase5-dev, qt5-qmake, qttools5-dev-tools)
4. CMake (versi 3.5 ke atas)

Semua ini sudah tersedia otomatis di GitHub Codespaces, jadi kamu tinggal jalankan!

### ✨ Catatan
1. Setiap kali kamu mengubah kode di rust_backend, jangan lupa build ulang cargo build --release supaya file .so terupdate.
2. Untuk mendebug error saat build Qt, lihat bagian error make di terminal Codespaces.
3. Jika tampilan GUI tidak muncul di Codespaces GUI secara default, jalankan Codespaces > Ports > Forward Port di menu.

### 📚 REFERENSI
2013 IEEE International Conference on Automation Science and Engineering (CASE). 2013.
Assagaf, Idrus, Jonri Lomi Ga, Agus Sukandi, Abdul Azis Abdillah, dan Samsul Arifin. 2023.
1 Recent in Engineering Science and Technology Machine Predictive Maintenance by
Using Support Vector Machines.

Guo, Gongde, Hui Wang, David Bell, Yaxin Bi, dan Kieran Greer. 2003. 2888 LNCS KNN
Model-Based Approach in Classification.

Han, Hyoil, Seungjin Lim, Kyoungwon Suh, Seonghyun Park, Seong Je Cho, dan Minkyu
Park. 2020. “Enhanced android malware detection: An SVM-based machine learning
approach.” Dalam Proceedings - 2020 IEEE International Conference on Big Data and
Smart Computing, BigComp 2020, Institute of Electrical and Electronics Engineers Inc.,
75–81. doi:10.1109/BigComp48618.2020.00-96.

Putra, Ikhsan Fachriansyah, Mahdhivan Syafwan, Monika Rianti Helmi, dan Admi Nazra.
2023a. “BENTUK EKSPLISIT RUMUS BEDA MAJU DAN BEDA MUNDUR UNTUK
TURUNAN KE-N DENGAN ORDE KETELITIAN KE-N BERDASARKAN DERET
TAYLOR.” Jurnal Lebesgue : Jurnal Ilmiah Pendidikan Matematika, Matematika dan
Statistika 4(3): 1675–86. doi:10.46306/lb.v4i3.461.

Putra, Ikhsan Fachriansyah, Mahdhivan Syafwan, Monika Rianti Helmi, dan Admi Nazra.
2023b. “BENTUK EKSPLISIT RUMUS BEDA MAJU DAN BEDA MUNDUR UNTUK
TURUNAN KE-N DENGAN ORDE KETELITIAN KE-N BERDASARKAN DERET
TAYLOR.” Jurnal Lebesgue : Jurnal Ilmiah Pendidikan Matematika, Matematika dan
Statistika 4(3): 1675–86. doi:10.46306/lb.v4i3.461.

Putra, Permana, Akim M H Pardede, dan Siswan Syahputra. 2022. “ANALISIS METODE K-
NEAREST NEIGHBOUR (KNN) DALAM KLASIFIKASI DATA IRIS BUNGA.”
Jurnal Teknik Informatika Kaputama (JTIK) 6(1).

Tak, Ping, dan Peter Tang. Table-Lookup Algorithms for Elez entary Functions and their Error
Analysis*.

Wu, Sze Jung, Nagi Gebraeel, Mark A. Lawley, dan Yuehwern Yih. 2007. “A neural network
integrated decision support system for condition-based optimal predictive maintenance
policy.” IEEE Transactions on Systems, Man, and Cybernetics Part A:Systems and
Humans 37(2): 226–36. doi:10.1109/TSMCA.2006.886368.

Yuan, Ruixi, Zhu Li, Xiaohong Guan, dan Li Xu. 2010. “An SVM-based machine learning
method for accurate Internet traffic classification.” Information Systems Frontiers 12(2):
149–56. doi:10.1007/s10796-008-9131-2.

Zhang, Shichao, Xuelong Li, Ming Zong, Xiaofeng Zhu, dan Debo Cheng. 2017. “Learning k
for kNN Classification.” ACM Transactions on Intelligent Systems and Technology 8(3).
doi:10.1145/2990508.
