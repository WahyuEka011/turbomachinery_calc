// knn_evaluation.rs
use super::knn::KNN;
use crate::knn::data::{load_dataset, DataPoint};

#[allow(dead_code)]
pub fn evaluate_knn(knn: &KNN, test_data: &[DataPoint]) {
    let mut correct = 0;
    let total = test_data.len();

    for sample in test_data {
        let prediction = knn.predict(sample);
        if prediction == sample.label {
            correct += 1;
        }
    }

    let accuracy = correct as f64 / total as f64 * 100.0;
    println!("Accuracy: {:.2}%", accuracy);
}

#[allow(dead_code)]
pub fn load_test_data() -> Vec<DataPoint> {
    load_dataset("data/test.csv")
}