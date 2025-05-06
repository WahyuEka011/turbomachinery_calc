// src/config.rs

#[allow(dead_code)]
pub struct Config {
    pub learning_rate: f32,
    pub epochs: usize,
    pub hidden_size1: usize,
    pub hidden_size2: usize,
    pub dataset_path_train: &'static str,
    pub dataset_path_val: &'static str,
    pub dataset_path_test: &'static str,
}

// Instance global dari konfigurasi
#[allow(dead_code)]
pub const CONFIG: Config = Config {
    learning_rate: 0.001,          // bisa kamu ubah sewaktu-waktu
    epochs: 5000,                   // jumlah epoch training
    hidden_size1: 32,              // hidden layer pertama
    hidden_size2: 24,              // hidden layer kedua
    dataset_path_train: "data/train.csv",
    dataset_path_val: "data/validation.csv",
    dataset_path_test: "data/test.csv",
};
