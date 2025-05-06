use crate::svm::data::DataPoint;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::error::Error;
use plotters::prelude::*;

#[allow(dead_code)]
pub struct SvmModel {
    weights: Vec<f64>,
    bias: f64,
    lr: f64,
    epochs: usize,
}

#[allow(dead_code)]
impl SvmModel {
    pub fn new(n_features: usize, lr: f64, epochs: usize) -> Self {
        Self {
            weights: vec![0.0; n_features],
            bias: 0.0,
            lr,
            epochs,
        }
    }

    pub fn train(&mut self, data: &[DataPoint], target_label: u8) {
        for _ in 0..self.epochs {
            for point in data {
                let y = if point.label == target_label { 1.0 } else { -1.0 };
                let mut dot = self.bias;
                for i in 0..self.weights.len() {
                    dot += self.weights[i] * point.features[i];
                }

                if y * dot < 1.0 {
                    for i in 0..self.weights.len() {
                        self.weights[i] += self.lr * (y * point.features[i]);
                    }
                    self.bias += self.lr * y;
                }
            }
        }
    }

    pub fn predict_label(&self, features: &[f64]) -> u8 {
        let mut dot = self.bias;
        for i in 0..self.weights.len() {
            dot += self.weights[i] * features[i];
        }
        if dot >= 0.0 {
            1
        } else {
            0
        }
    }
}

#[allow(dead_code)]
pub fn evaluate_accuracy(model: &SvmModel, test_data: &[DataPoint]) -> f64 {
    let correct = test_data.iter()
        .filter(|dp| model.predict_label(&dp.features) == dp.label)
        .count();
    correct as f64 / test_data.len() as f64 * 100.0
}

#[allow(dead_code)]
pub fn save_predictions(model: &SvmModel, data: &[DataPoint], path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "electricity,rpm,pressure_steam,vibration,label_asli,prediksi")?;
    for dp in data {
        let prediction = model.predict_label(&dp.features);
        writeln!(
            writer,
            "{},{},{},{},{},{}",
            dp.features[0], dp.features[1], dp.features[2], dp.features[3],
            dp.label, prediction
        )?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn plot_predictions(data: &[DataPoint], model: &SvmModel, path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (x_min, x_max) = data.iter().map(|d| d.features[0])
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), val| (min.min(val), max.max(val)));
    let (y_min, y_max) = data.iter().map(|d| d.features[1])
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), val| (min.min(val), max.max(val)));

    let mut chart = ChartBuilder::on(&root)
        .caption("Hasil Prediksi SVM (fitur 1 vs 2)", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    for dp in data {
        let x = dp.features[0];
        let y = dp.features[1];
        let pred = model.predict_label(&dp.features);
        let color = match (dp.label, pred) {
            (0, 0) | (1, 1) => &GREEN,
            _ => &RED,
        };
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, color.filled())))?;
    }

    Ok(())
}
