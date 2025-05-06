use std::path::Path;
use csv::ReaderBuilder;
// use std::fs::File;
// use std::io::{self, BufReader, Read};

// Struct untuk DataPoint yang memuat fitur dan label
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DataPoint {
    pub features: Vec<f32>,
    pub label: String,
}

// Fungsi untuk memuat dataset dari file CSV
#[allow(dead_code)]
pub fn load_dataset<P: AsRef<Path>>(path: P) -> Vec<DataPoint> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path).expect("Unable to read file");
    
    let mut dataset = Vec::new();
    
    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        
        // Mengambil 4 kolom pertama sebagai fitur
        let features: Vec<f32> = record.iter()
            .take(4)  // Hanya ambil fitur (col 1-4: electricity, rpm, pressure_steam, vibration)
            .map(|s| s.parse::<f32>().expect("Invalid float"))
            .collect();

        // Kolom ke-5 adalah label (status)
        let label = record[4].to_string();
        
        dataset.push(DataPoint { features, label });
    }
    
    dataset
}

// Fungsi untuk memuat data uji dari file CSV
#[allow(dead_code)]
pub fn load_test_data<P: AsRef<Path>>(path: P) -> Vec<DataPoint> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path).expect("Unable to read file");
    
    let mut test_data = Vec::new();
    
    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        
        // Mengambil 4 kolom pertama sebagai fitur
        let features: Vec<f32> = record.iter()
            .take(4)  // Hanya ambil fitur (col 1-4: electricity, rpm, pressure_steam, vibration)
            .map(|s| s.parse::<f32>().expect("Invalid float"))
            .collect();

        // Kolom ke-5 adalah label (status)
        let label = record[4].to_string();
        
        test_data.push(DataPoint { features, label });
    }
    
    test_data
}
