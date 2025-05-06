mod data;
mod model;
mod config;
mod utils;
mod visualize;
mod predict;

use std::path::Path;
use config::*;
use model::*;
use utils::*;
use ndarray::Array2;
use data::{load_dataset, normalize_dataset};
use std::collections::VecDeque;
use utils::{get_user_input, read_user_input};
use std::fs::File;
use std::io::Write;
use predict::run_prediction;
use visualize::visualize_predictions;



fn main() {
    // Load dataset training, validation, dan test
    let (mut x_train, y_train) = load_dataset("data/train.csv").expect("Gagal load train.csv");
    let (mut x_val, y_val) = load_dataset("data/validation.csv").expect("Gagal load validation.csv");
    let (mut x_test, y_test) = load_dataset("data/test.csv").expect("Gagal load test.csv");

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
        model = NeuralNetwork::new(4, 24, 16, 3);
        model.train(&x_train, &y_train, 0.01, 10000);

        // Evaluasi akurasi
        let acc = model.evaluate(&x_val, &y_val);
        println!("📈 Akurasi pada validation set: {:.2}%", acc * 100.0);

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

    // Tambahan notifikasi
    println!("📥 Data berhasil dibaca dan dinormalisasi...");
    println!("🧠 Model selesai melakukan prediksi.");
    println!("✅ Akurasi model: {:.2}%", test_acc);

    // Setelah evaluasi akurasi test set:
    let mut pred_labels = Vec::new();
    let mut true_labels = Vec::new();
    let mut confidences = Vec::new();

    for i in 0..x_test.nrows() {
        let input = x_test.row(i).to_owned();
        let pred = model.forward(&input);

        let predicted_class = pred
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let confidence = *pred
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        pred_labels.push(predicted_class as u32);
        true_labels.push(y_test[i]);
        confidences.push(confidence);
    }

    // Buat direktori output jika belum ada
    std::fs::create_dir_all("output").expect("Gagal membuat folder output");

    // Simpan hasil ke CSV
    let mut file = File::create("output/test_predictions.csv").expect("Gagal membuat file prediksi");
    writeln!(file, "TrueLabel,PredictedLabel,Confidence").unwrap();

    for i in 0..pred_labels.len() {
        writeln!(
            file,
            "{},{},{}",
            true_labels[i], pred_labels[i], confidences[i]
        )
        .unwrap();
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

    let label_map = ["Normal", "Maintenance", "Fail"];
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

        let input2d = input.insert_axis(Axis(0));
        let normalized = preprocess(&input2d);
        let output = model.forward(&normalized.row(0).to_owned());
        println!("🧪 Output prediksi langsung: {:?}", output);

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
        // Jalankan prediksi (hasil disimpan ke CSV)
        run_prediction();
}
