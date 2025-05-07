use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;
use ndarray::{Array2, Array1};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "surface_temp")]
    pub surface_temp: f32,
    #[serde(rename = "steam")]
    pub steam: f32,
    #[serde(rename = "rpm")]
    pub rpm: f32,
    #[serde(rename = "torque")]
    pub torque: f32,
    #[serde(rename = "vibration")]
    pub vibration: f32,
    #[serde(rename = "Failure Type")]
    pub failure_type: String,
}

fn label_to_numeric(label: &str) -> f32 {
    match label.trim() {
        "No Failure" => 0.0,
        "Power Failure" => 1.0,
        "Overheat" => 2.0,
        _ => panic!("Label status tidak dikenali: {}", label),
    }
}

pub fn load_dataset<P: AsRef<Path>>(path: P) -> Result<(Array2<f32>, Array1<f32>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(path)?);

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        // Uncomment ini untuk debug isi record
        // println!("{:?}", record);

        features.push(vec![
            record.surface_temp,
            record.steam,
            record.rpm,
            record.torque,
            record.vibration,
        ]);

        labels.push(label_to_numeric(&record.failure_type));
    }

    if features.is_empty() {
        return Err("Dataset kosong setelah parsing!".into());
    }

    println!("Total sample: {}, Jumlah fitur per sample: {}", features.len(), features[0].len());

    let x = Array2::from_shape_vec((features.len(), 5), features.concat())?;
    let y = Array1::from_vec(labels);

    Ok((x, y))
}
