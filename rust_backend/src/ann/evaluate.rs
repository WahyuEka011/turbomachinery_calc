use ndarray::{Array1, Array2, Axis};
use crate::ann::model::NeuralNetwork;
use std::io::Result;

pub fn evaluate_accuracy(model: &NeuralNetwork, x: &Array2<f32>, y: &Array1<f32>) -> f32 {
    let mut correct = 0;
    for i in 0..x.nrows() {
        let input = x.row(i).to_owned();
        let pred = model.forward(&input);
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

pub fn evaluate_and_save_predictions(
    model: &NeuralNetwork,
    x: &Array2<f32>,
    y: &Array1<f32>,
    output_path: &str,
) -> Result<f32> {
    let mut correct = 0;
    let mut writer = csv::Writer::from_path(output_path)?;

    writer.write_record(&["TrueLabel", "PredictedLabel", "Confidence"])?;

    for i in 0..x.nrows() {
        let input = x.row(i).to_owned();
        let pred = model.forward(&input);
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
