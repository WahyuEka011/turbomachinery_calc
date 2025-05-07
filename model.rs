use ndarray::{Array1, Array2, Axis};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// Fungsi aktivasi ReLU
fn relu(x: &Array1<f32>) -> Array1<f32> {
    x.mapv(|v| v.max(0.0))
}

/// Turunan dari ReLU (untuk backprop)
fn relu_derivative(x: &Array1<f32>) -> Array1<f32> {
    x.mapv(|v| if v > 0.0 { 1.0 } else { 0.0 })
}

/// Softmax untuk output klasifikasi
fn softmax(x: &Array1<f32>) -> Array1<f32> {
    let max = x.fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let exp: Array1<f32> = x.mapv(|v| (v - max).exp() + 1e-8);
    let sum = exp.sum();
    exp / sum
}

/// Cross entropy loss
fn cross_entropy(pred: &Array1<f32>, target: usize) -> f32 {
    let epsilon = 1e-10;
    -((pred[target] + epsilon).ln())
}

pub struct AdamOptimizer {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    m: Array2<f32>,  // Moment pertama
    v: Array2<f32>,  // Moment kedua
    t: usize,        // Timestep
}

impl AdamOptimizer {
    pub fn new(learning_rate: f32, beta1: f32, beta2: f32, epsilon: f32, shape: (usize, usize)) -> Self {
        AdamOptimizer {
            learning_rate,
            beta1,
            beta2,
            epsilon,
            m: Array2::<f32>::zeros(shape),
            v: Array2::<f32>::zeros(shape),
            t: 0,  // Initialize timestep
        }
    }

    pub fn step(&mut self, weights: &mut Array2<f32>, grads: &Array2<f32>) {
        self.t += 1;
        
        // Update m and v with element-wise operations
        self.m = self.beta1 * &self.m + (1.0 - self.beta1) * grads;
        self.v = self.beta2 * &self.v + (1.0 - self.beta2) * grads.mapv(|x| x.powi(2));
        
        // Compute m_hat and v_hat
        let m_hat = &self.m / (1.0 - self.beta1.powi(self.t as i32));
        let v_hat = &self.v / (1.0 - self.beta2.powi(self.t as i32));
        
        // Update the weights
        *weights -= &(self.learning_rate * &m_hat / (&v_hat.mapv(|x| x.sqrt()) + self.epsilon));
    }
    
    
}

/// Struktur ANN
#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub weights_input_hidden1: Array2<f32>,
    pub bias_hidden1: Array1<f32>,
    pub weights_hidden1_hidden2: Array2<f32>,
    pub bias_hidden2: Array1<f32>,
    pub weights_hidden2_hidden3: Array2<f32>,  // Tambahkan bobot hidden2 ke hidden3
    pub bias_hidden3: Array1<f32>,             // Tambahkan bias untuk hidden3
    pub weights_hidden_output: Array2<f32>,
    pub bias_output: Array1<f32>,
}

impl NeuralNetwork {
    /// Inisialisasi ANN
    pub fn new(input_size: usize, hidden_size1: usize, hidden_size2: usize, hidden_size3: usize, output_size: usize) -> Self {
        let dist = Uniform::new(-0.1, 0.1);
        Self {
            weights_input_hidden1: Array2::random((hidden_size1, input_size), dist),
            bias_hidden1: Array1::zeros(hidden_size1),
            weights_hidden1_hidden2: Array2::random((hidden_size2, hidden_size1), dist),
            bias_hidden2: Array1::zeros(hidden_size2),
            weights_hidden2_hidden3: Array2::random((hidden_size3, hidden_size2), dist),  // Inisialisasi layer ke-3
            bias_hidden3: Array1::zeros(hidden_size3),  // Bias untuk layer ke-3
            weights_hidden_output: Array2::random((output_size, hidden_size3), dist),
            bias_output: Array1::zeros(output_size),
        }
    }

    /// Forward propagation
    pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        let z1 = self.weights_input_hidden1.dot(input) + &self.bias_hidden1;
        let a1 = relu(&z1);

        let z2 = self.weights_hidden1_hidden2.dot(&a1) + &self.bias_hidden2;
        let a2 = relu(&z2);

        let z3 = self.weights_hidden2_hidden3.dot(&a2) + &self.bias_hidden3;  // Tambahkan layer ke-3
        let a3 = relu(&z3);  // Aktivasi layer ke-3

        let z4 = self.weights_hidden_output.dot(&a3) + &self.bias_output;  // Output dari hidden3 ke output
        softmax(&z4)
    }

    /// Training 1 epoch (manual SGD)
    pub fn train(&mut self, x: &Array2<f32>, y: &Array1<f32>, learning_rate: f32, epochs: usize) {
        let label_names = ["No Failure", "Power Failure", "Overheat"];
        let num_samples = x.len_of(Axis(0));

        for epoch in 0..epochs {
            println!("📚 Epoch {}/{}", epoch + 1, epochs);
            let mut total_loss = 0.0;

            for i in 0..num_samples {
                let input = x.row(i).to_owned();
                let target = y[i] as usize;

                // === Forward ===
                let z1 = self.weights_input_hidden1.dot(&input) + &self.bias_hidden1;
                let a1 = relu(&z1);
                let z2 = self.weights_hidden1_hidden2.dot(&a1) + &self.bias_hidden2;
                let a2 = relu(&z2);
                let z3 = self.weights_hidden2_hidden3.dot(&a2) + &self.bias_hidden3;  // Forward ke hidden3
                let a3 = relu(&z3);  // Aktivasi hidden3
                let z4 = self.weights_hidden_output.dot(&a3) + &self.bias_output;
                let pred = softmax(&z4);

                let loss = cross_entropy(&pred, target);
                total_loss += loss;

                if i % 500 == 0 || i == num_samples - 1 || !loss.is_finite() {
                    println!("Sample {}: Label = {}, Loss = {:.4}", i, label_names[target], loss);
                }

                // === Backpropagation ===
                let mut d_z4 = pred.clone();
                d_z4[target] -= 1.0;

                let d_w4 = d_z4.view().insert_axis(Axis(1)).dot(&a3.view().insert_axis(Axis(0)));
                let d_b4 = d_z4.clone();

                let d_a3 = self.weights_hidden_output.t().dot(&d_z4);
                let d_z3 = d_a3 * relu_derivative(&z3);
                let d_w3 = d_z3.view().insert_axis(Axis(1)).dot(&a2.view().insert_axis(Axis(0)));
                let d_b3 = d_z3.clone();

                let d_a2 = self.weights_hidden2_hidden3.t().dot(&d_z3);
                let d_z2 = d_a2 * relu_derivative(&z2);
                let d_w2 = d_z2.view().insert_axis(Axis(1)).dot(&a1.view().insert_axis(Axis(0)));
                let d_b2 = d_z2.clone();

                let d_a1 = self.weights_hidden1_hidden2.t().dot(&d_z2);
                let d_z1 = d_a1 * relu_derivative(&z1);
                let d_w1 = d_z1.view().insert_axis(Axis(1)).dot(&input.view().insert_axis(Axis(0)));
                let d_b1 = d_z1.clone();

                // === Update bobot ===
                self.weights_hidden_output -= &(learning_rate * d_w4);
                self.bias_output -= &(learning_rate * d_b4);

                self.weights_hidden2_hidden3 -= &(learning_rate * d_w3);
                self.bias_hidden3 -= &(learning_rate * d_b3);

                self.weights_hidden1_hidden2 -= &(learning_rate * d_w2);
                self.bias_hidden2 -= &(learning_rate * d_b2);

                self.weights_input_hidden1 -= &(learning_rate * d_w1);
                self.bias_hidden1 -= &(learning_rate * d_b1);
            }

            let avg_loss = total_loss / num_samples as f32;
            println!("📉 Rata-rata Loss epoch {}: {:.4}", epoch + 1, avg_loss);
            println!("--------------------------------------------------");
        }
    }
    /// Save model ke file
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    /// Load model dari file
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let nn = serde_json::from_reader(reader)?;
        Ok(nn)
    }

    /// Prediksi dengan output RUL (Remaining Useful Life)
    /// Prediksi dengan output status + RUL + confidence
    pub fn predict_with_rul(&self, input: &Array1<f32>, recent_preds: &mut VecDeque<usize>) {
    // Lakukan forward pass
    let output = self.forward(input);

    // Ambil status indeks dan confidence tertinggi
    let (status_idx, confidence_val) = output
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, &val)| (idx, val))
        .unwrap_or((2, 0.0)); // default ke Fail kalau error

    // Tambahkan ke daftar prediksi terbaru
    recent_preds.push_back(status_idx);
    if recent_preds.len() > 10 {
        recent_preds.pop_front();
    }

    // Tampilkan hasil prediksi dengan warna dan emoji
    match status_idx {
        0 => {
            println!("\x1b[32m✅ Status Prediksi: No Failure ✅\x1b[0m");
            println!("🔧 RUL menuju Maintenance: {:.2} jam", 30.0);
        }
        1 => {
            println!("\x1b[33m⚠️ Status Prediksi: Power Failure ⚠️\x1b[0m");
            println!("⚠️ RUL menuju Failure: {:.2} jam", 10.0);
        }
        2 => {
            println!("\x1b[31m⛔ Status Prediksi: Overheat ❌\x1b[0m");
            println!("❌ Sistem sudah dalam kondisi FAIL. RUL tidak tersedia.");
        }
        _ => {
            println!("Status tidak diketahui");
        }
    }

    // Tampilkan confidence
    println!("📊 Tingkat kepercayaan model: {:.2}%", confidence_val * 100.0);
    }


        /// Evaluasi akurasi model
        pub fn evaluate(&self, x: &Array2<f32>, y: &Array1<f32>) -> f32 {
        let mut correct = 0;
        for i in 0..x.nrows() {
            let input = x.row(i).to_owned();
            let pred = self.forward(&input);
            let predicted_class = pred
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            if predicted_class == y[i] as usize {
                correct += 1;
            }
        }
        correct as f32 / x.nrows() as f32
    }

        /// Evaluasi dan simpan hasil prediksi ke file CSV
        #[allow(dead_code)]
        pub fn evaluate_and_save_predictions(
        &self,
        x: &Array2<f32>,
        y: &Array1<f32>,
        output_path: &str,
        ) -> std::io::Result<f32> {
    let mut correct = 0;
    let mut writer = csv::WriterBuilder::new().has_headers(true).from_path(output_path)?;

    // Header CSV
    writer.write_record(&["TrueLabel", "PredictedLabel", "Confidence"])?;

    for i in 0..x.nrows() {
        let input = x.row(i).to_owned();
        let pred = self.forward(&input);
        let predicted_class = pred
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();

        let confidence = pred[predicted_class];

        if predicted_class == y[i] as usize {
            correct += 1;
        }

        writer.write_record(&[
            y[i].to_string(),
            predicted_class.to_string(),
            format!("{:.4}", confidence),
        ])?;
    }

        writer.flush()?;

        Ok(correct as f32 / x.nrows() as f32)
    }

    

}
