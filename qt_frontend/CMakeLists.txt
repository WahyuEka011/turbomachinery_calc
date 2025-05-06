cmake_minimum_required(VERSION 3.5)
project(QtRustIntegration)

set(CMAKE_CXX_STANDARD 11)

# Aktifkan fitur otomatis dari Qt
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

# Cari Qt5
find_package(Qt5 COMPONENTS Widgets REQUIRED)

# ================================
# Bagian integrasi Rust
# ================================
# Path ke hasil build Rust (.so)
set(RUST_LIB_PATH "${CMAKE_SOURCE_DIR}/../rust_backend/target/release")

# Header dan library Rust (kalau ada)
include_directories(${CMAKE_CURRENT_SOURCE_DIR} ${RUST_LIB_PATH})

# Menambahkan direktori tempat library Rust berada
link_directories(${RUST_LIB_PATH})

# Nama library Rust TANPA `lib` dan TANPA `.so`
# Misal: libturbomachinery_calc.so → turbomachinery_calc
# Pastikan nama ini sesuai dengan nama file `.so` yang dihasilkan oleh Rust (contoh: librust_backend.so)
set(RUST_LIB rust_backend)

# ================================
# File UI (kalau pakai uic)
# ================================
set(UI_FILES
    mainwindow.ui
    annwindow.ui
    homepagewindow.ui
)

# ================================
# File source Qt
# ================================
add_executable(QtRustIntegration
    main.cpp
    mainwindow.cpp
    mainwindow.h
    homepagewindow.cpp
    homepagewindow.h
    annwindow.cpp
    annwindow.h
    ${UI_FILES}
)

# ================================
# Linking Qt dan Rust
# ================================
target_link_libraries(QtRustIntegration
    PRIVATE
    Qt5::Widgets
    ${RUST_LIB}  # Menambahkan library Rust
)

# Tambahan untuk header hasil .ui
include_directories(${CMAKE_CURRENT_BINARY_DIR})

# Set properti (optional, bisa dihapus jika tidak pakai fitur khusus)
set_target_properties(QtRustIntegration PROPERTIES
    INCLUDE_DIRECTORIES "${CMAKE_CURRENT_BINARY_DIR}"
)
