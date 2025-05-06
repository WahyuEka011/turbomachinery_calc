use crate::knn::data::DataPoint;
use crate::knn::utils::euclidean_distance;
use std::collections::HashMap;

pub struct KNN {
    k: usize,                // Jumlah tetangga terdekat
    training_data: Vec<DataPoint>, // Data pelatihan
}

impl KNN {
    // Membuat instansi KNN dengan k yang ditentukan
    #[allow(dead_code)]
    pub fn new(k: usize) -> Self {
        KNN {
            k,
            training_data: Vec::new(),
        }
    }

    // Melatih model kNN dengan data pelatihan
    #[allow(dead_code)]
    pub fn train(&mut self, data: Vec<DataPoint>) {
        self.training_data = data;
    }

    // Prediksi label untuk data baru
    pub fn predict(&self, test_point: &DataPoint) -> String {
        let mut distances: Vec<(f64, String)> = self
            .training_data
            .iter()
            .map(|dp| {
                let dist = euclidean_distance(&test_point.features, &dp.features);
                (dist, dp.label.clone())
            })
            .collect();
    
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
        let nearest_k = &distances[..self.k];
    
        let mut label_count = HashMap::new();
        for (_, label) in nearest_k {
            *label_count.entry(label.clone()).or_insert(0) += 1;
        }
    
        label_count
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(label, _)| label)
            .unwrap_or_else(|| "Unknown".to_string())
        
    }
}
