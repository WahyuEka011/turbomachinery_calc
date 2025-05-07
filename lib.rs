// src/lib.rs
use pyo3::{prelude::*, exceptions::PyIOError, types::PyAny};
use ndarray::Array1;
use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::lazy_static;

mod ann;

// Global storage for Qt callback
lazy_static! {
    static ref QT_CALLBACK: Mutex<Option<Py<PyAny>>> = Mutex::new(None);
}

#[pyfunction]
fn set_qt_callback(callback: Py<PyAny>) -> PyResult<()> {
    let mut cb = QT_CALLBACK.lock().unwrap();
    *cb = Some(callback);
    Ok(())
}

#[pyfunction]
fn predict(input: Vec<f32>) -> PyResult<Vec<f32>> {
    let model = crate::ann::model::NeuralNetwork::load("model.json")
        .map_err(|e| PyIOError::new_err(e.to_string()))?;
    let input_arr = Array1::from(input);
    let output = model.forward(&input_arr);
    
    // Notify Qt if callback is set
    if let Some(callback) = QT_CALLBACK.lock().unwrap().as_ref() {
        Python::with_gil(|py| {
            callback.call1(py, (format!("Prediction completed: {:?}", output),))
                .map_err(|e| PyIOError::new_err(e.to_string()))?;
            Ok::<(), PyErr>(())
        })?;
    }
    
    Ok(output.to_vec())
}

#[pyfunction]
fn predict_with_rul(input: Vec<f32>) -> PyResult<String> {
    let model = crate::ann::model::NeuralNetwork::load("model.json")
        .map_err(|e| PyIOError::new_err(e.to_string()))?;
    let input_arr = Array1::from(input);
    let mut recent_preds = VecDeque::new();
    let result = model.predict_with_rul(&input_arr, &mut recent_preds);
    
    // Notify Qt with RUL prediction
    if let Some(callback) = QT_CALLBACK.lock().unwrap().as_ref() {
        Python::with_gil(|py| {
            callback.call1(py, (format!("RUL Prediction: {:?}", result),))
                .map_err(|e| PyIOError::new_err(e.to_string()))?;
            Ok::<(), PyErr>(())
        })?;
    }
    
    Ok("Prediction with RUL completed".to_string())
}

#[pymodule]
fn turbomachinery_qtpyth(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_qt_callback, m)?)?;
    m.add_function(wrap_pyfunction!(predict, m)?)?;
    m.add_function(wrap_pyfunction!(predict_with_rul, m)?)?;
    Ok(())
}
