use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;
use ndarray::{Array2, Array1};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Record {
    pub electricity: f32,
    pub rpm: f32,
    pub pressure_steam: f32,
    pub vibration: f32,
    pub status: String,
}

/// Mengonversi status ke angka: Normal = 0, Maintenance = 1, Fail = 2
#[allow(dead_code)]
fn label_to_numeric(label: &str) -> f32 {
    match label {
        "Normal" => 0.0,
        "Maintenance" => 1.0,
        "Fail" => 2.0,
        _ => panic!("Label status tidak dikenali: {}", label),
    }
}

/// Fungsi untuk load CSV dan mengembalikan tuple (X, y)
#[allow(dead_code)]
pub fn load_dataset<P: AsRef<Path>>(path: P) -> Result<(Array2<f32>, Array1<f32>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(path)?);

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        // Fitur asli
        let electricity = record.electricity;
        let rpm = record.rpm.max(1.0); // Hindari log(0)
        let pressure = record.pressure_steam;
        let vibration = record.vibration;

        // Gabungkan semua fitur
        features.push(vec![
            electricity,
            rpm,
            pressure,
            vibration,
        ]);

        // Label
        labels.push(label_to_numeric(&record.status));
    }

    println!("Total sample: {}, Jumlah fitur per sample: {}", features.len(), features[0].len());

    let x = Array2::from_shape_vec((features.len(), 4), features.concat())?;
    let y = Array1::from_vec(labels);

    Ok((x, y))
}
