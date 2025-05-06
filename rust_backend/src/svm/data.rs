use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DataPoint {
    pub features: Vec<f64>,
    pub label: u8,
}

#[allow(dead_code)]
pub fn load_dataset(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut dataset = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() < 5 {
            println!("Baris ke-{} tidak valid: {:?}", i + 1, parts);
            continue;
        }
        let features: Vec<f64> = parts[0..4]
            .iter()
            .map(|v| v.parse::<f64>().unwrap_or(0.0))
            .collect();
        let label: u8 = parts[4].parse().unwrap_or(0);
        dataset.push(DataPoint { features, label });
    }

    Ok(dataset)
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Scaler {
    min: Vec<f64>,
    max: Vec<f64>,
}

#[allow(dead_code)]
impl Scaler {
    pub fn fit(data: &[DataPoint]) -> Self {
        let num_features = data[0].features.len();
        let mut min = vec![f64::INFINITY; num_features];
        let mut max = vec![f64::NEG_INFINITY; num_features];

        for point in data {
            for (i, &val) in point.features.iter().enumerate() {
                if val < min[i] {
                    min[i] = val;
                }
                if val > max[i] {
                    max[i] = val;
                }
            }
        }

        Scaler { min, max }
    }

    pub fn transform(&self, data: &mut [DataPoint]) {
        for point in data.iter_mut() {
            for i in 0..point.features.len() {
                let min = self.min[i];
                let max = self.max[i];
                point.features[i] = if (max - min).abs() > 1e-8 {
                    (point.features[i] - min) / (max - min)
                } else {
                    0.0
                };
            }
        }
    }

    pub fn fit_transform(data: &mut [DataPoint]) -> Self {
        let scaler = Self::fit(data);
        scaler.transform(data);
        scaler
    }

    pub fn apply_scaler(data: &mut [DataPoint], scaler: &Scaler) {
        scaler.transform(data);
    }
}
