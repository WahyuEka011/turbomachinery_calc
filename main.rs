mod data;
mod model;
mod config;
mod utils;
mod visualize;
mod normalization;

use std::path::Path;
use config::*;
use model::*;
use utils::*;
use ndarray::{Array1, Array2};
use data::{load_dataset};
use std::collections::VecDeque;
use utils::{get_user_input, read_user_input};
use std::fs::File;
use std::io::Write;
use normalization::normalize_dataset;
use model::{AdamOptimizer}; // Pastikan untuk memasukkan AdamOptimizer dari model.rs


fn main() {
    // Inisialisasi data input dan target
    let input_size = 5; // Misalnya 5 input features
    let output_size = 3; // Misalnya 3 kelas (label)

    let mut model_weights = Array2::<f32>::zeros((input_size, output_size)); // Inisialisasi bobot model

    // Inisialisasi Adam optimizer
    let mut adam = AdamOptimizer::new(0.0005, 0.9, 0.999, 1e-8, (input_size, output_size));

    let epochs = 2000;

    // Training loop
    for epoch in 0..epochs {
        let grads = compute_gradients(&model_weights); // Hitung gradien (implementasi perhitungan gradien diperlukan)
        adam.step(&mut model_weights, &grads); // Update bobot menggunakan Adam

        if epoch % 100 == 0 {
            let loss = compute_loss(&model_weights); // Hitung loss (implementasi perhitungan loss diperlukan)
            println!("Epoch {}: Loss = {}", epoch, loss);
        }
    }
    
    // Fungsi untuk menghitung gradien (implementasi diperlukan)
    fn compute_gradients(weights: &Array2<f32>) -> Array2<f32> {
        // Implementasikan perhitungan gradien
        Array2::<f32>::zeros(weights.dim())
    }
    
    // Fungsi untuk menghitung loss (implementasi diperlukan)
    fn compute_loss(weights: &Array2<f32>) -> f32 {
        // Implementasikan perhitungan loss
        0.0
    }

    // Load dataset training, validation, dan test
    let (mut x_train, y_train) = load_dataset("data/train.csv").expect("Gagal load train.csv");
    println!("Data Train: {:?}", x_train);
    
    let (mut x_val, y_val) = load_dataset("data/validation.csv").expect("Gagal load validation.csv");
    println!("Data Validation: {:?}", x_val);
    
    let (mut x_test, y_test) = load_dataset("data/test.csv").expect("Gagal load test.csv");
    println!("Data Test: {:?}", x_test);

    // Normalisasi dataset
    x_train = normalize_dataset(&x_train);
    x_val = normalize_dataset(&x_val);
    x_test = normalize_dataset(&x_test);
 
    // Path model
    let model_path = "model.json";

    // Cek apakah model sudah ada
    let mut model: NeuralNetwork;
    if Path::new(model_path).exists() {
        println!("📦 Model ditemukan, memuat dari '{}'", model_path);
        model = NeuralNetwork::load(model_path).expect("Gagal memuat model.");
    } else {
        println!("⚙️ Model belum ditemukan. Training model baru...");

        // Inisialisasi dan training
        model = NeuralNetwork::new(5, 8, 8, 8, 3);
        model.train(&x_train, &y_train, 0.001, 2000);

        // Prediksi setelah training
        let input = Array1::from(vec![0.5, 0.2, 0.3, 0.7, 0.1]); // Input baru untuk prediksi dengan 5 nilai
        let result = model.forward(&input);
        println!("Prediksi output: {:?}", result);

        // Evaluasi akurasi
        let acc = model.evaluate(&x_val, &y_val);
        println!("📈 Akurasi pada validation set: {:.4}%", acc * 100.0);

        // Simpan model
        model.save(model_path).expect("Gagal menyimpan model");
        println!("✅ Model disimpan ke '{}'", model_path);
    }

    // Evaluasi akurasi pada test set
    let mut correct = 0;
    for i in 0..x_test.nrows() {
        let input = x_test.row(i).to_owned();
        let pred = model.forward(&input);
        let predicted_class = pred
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        if predicted_class == y_test[i] as usize {
            correct += 1;
        }
    }

    let test_acc = correct as f32 / x_test.nrows() as f32 * 100.0;
    println!("📊 Akurasi pada test set: {:.2}%", test_acc);

    // Buat direktori output jika belum ada
    std::fs::create_dir_all("output").expect("Gagal membuat folder output");

    // Simpan hasil ke CSV
    let mut file = File::create("output/test_predictions.csv").expect("Gagal membuat file prediksi");
    writeln!(file, "TrueLabel,PredictedLabel,Confidence").unwrap();

    for i in 0..x_test.nrows() {
        let input = x_test.row(i).to_owned();
        let pred = model.forward(&input);

        let predicted_class = pred
            .iter()
            .enumerate()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let confidence = *pred
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        writeln!(file, "{},{},{}", y_test[i], predicted_class, confidence).unwrap();
    }

    println!("📄 File output/test_predictions.csv berhasil dibuat untuk visualisasi!");

    // Panggil visualisasi scatter plot confidence prediksi
    if let Err(e) = visualize::visualize_predictions("output/test_predictions.csv", "output/prediksi_scatter.png") {
        eprintln!("❌ Gagal membuat scatter plot: {}", e);
    } else {
        println!("🖼️ Scatter plot berhasil disimpan di 'output/prediksi_scatter.png'");
    }

    // Kumpulkan confidence & label prediksi untuk visualisasi tambahan (line/bar plot misalnya)
    let mut confidence_data: Vec<(usize, f32, usize)> = Vec::new();

    for i in 0..x_test.nrows() {
        let input = x_test.row(i).to_owned();
        let pred = model.forward(&input);

        // Confidence = probabilitas tertinggi
        let (predicted_class, confidence) = pred.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, &conf)| (idx, conf))
            .unwrap();

        confidence_data.push((i, confidence, predicted_class));
    }

    // Panggil fungsi visualisasi tambahan (dari utils)
    if let Err(e) = utils::plot_prediction_confidence(&confidence_data) {
        eprintln!("Gagal membuat plot: {:?}", e);
    }

    // Prediksi contoh terakhir di test set dengan RUL
    let mut recent_preds = VecDeque::new();
    let test_sample = x_test.row(x_test.nrows() - 1).to_owned();
    model.predict_with_rul(&test_sample, &mut recent_preds);

    // Prediksi satu sampel terakhir dari training
    let sample = x_train.row(x_train.nrows() - 1).to_owned();
    let pred = model.forward(&sample);
    println!("Prediksi setelah training: {:?}", pred);

    let predicted_index = pred
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();

    let label_map = ["No Failure", "Power Failure", "Overheat"];
    let label_str = label_map[predicted_index];
    println!("📌 Prediksi Kelas: {} (label: {})", predicted_index, label_str);

    // Simpan model (lagi untuk jaga-jaga setelah evaluasi)
    model.save(model_path).expect("Gagal menyimpan model");

    // Load ulang model untuk verifikasi
    let loaded_model = NeuralNetwork::load(model_path).expect("Gagal load model");

    println!("=== 🧠 Predictive Maintenance Turbomachinery ===");
    println!("Silakan masukkan data sensor baru dari terminal:");

    let input_data = get_user_input();
    model.predict_with_rul(&input_data, &mut recent_preds);

    // Prediksi ulang untuk verifikasi
    let sample = x_test.row(0).to_owned();
    let pred = loaded_model.forward(&sample);
    println!("Prediksi dari model terload: {:?}", pred);

    // Loop interaktif
    loop {
        println!("\n=== 🧠 Predictive Maintenance Turbomachinery ===");
        println!("Silakan masukkan data sensor baru dari terminal:");

        let mut recent_preds = VecDeque::new();
        let input = read_user_input(); // Dari utils.rs
        model.predict_with_rul(&input, &mut recent_preds);

        println!("\nIngin memasukkan data lagi? (y/n): ");
        let mut lanjut = String::new();
        std::io::stdin()
            .read_line(&mut lanjut)
            .expect("Gagal baca input");
        let lanjut = lanjut.trim().to_lowercase();
        if lanjut != "y" {
            println!("🚪 Terima kasih sudah menggunakan sistem prediksi ini! 🙌");
            break;
        }
    }
}
