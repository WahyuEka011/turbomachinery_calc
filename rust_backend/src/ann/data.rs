use ndarray::{Array2, Array1, Axis};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn load_dataset(path: &str) -> Result<(Array2<f32>, Array1<f32>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut features = Vec::new();
    let mut labels = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue; // Skip header
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 5 {
            continue; // Harus 4 fitur + 1 label
        }

        let feat: Vec<f32> = parts[0..4]
            .iter()
            .map(|s| s.parse::<f32>().unwrap_or(0.0))
            .collect();

        let label_str = parts[4].trim(); // Ambil label (status)
        let label = match label_str {
            "Normal" => 0.0,       // Label "Normal" menjadi 0.0
            "Maintenance" => 1.0,  // Label "Maintenance" menjadi 1.0
            "Fail" => 2.0,         // Label "Fail" menjadi 2.0
            _ => panic!("Label tidak dikenali: {}", label_str), // Jika label tidak valid, tampilkan pesan error
        };

        features.push(feat);
        labels.push(label);
    }

    // Mengonversi fitur menjadi Array2 dan label menjadi Array1
    let x = Array2::from_shape_vec((features.len(), 4), features.concat())?;
    let y = Array1::from_vec(labels);

    Ok((x, y))
}

#[allow(dead_code)]
pub fn normalize_dataset(x: &Array2<f32>) -> Array2<f32> {
    let mean = x.mean_axis(Axis(0)).unwrap();
    let std = x.std_axis(Axis(0), 0.0);

    let normalized_rows: Vec<Array1<f32>> = x
        .rows()
        .into_iter()
        .map(|row| (&row - &mean) / &std)
        .collect();

    ndarray::stack(Axis(0), &normalized_rows.iter().map(|r| r.view()).collect::<Vec<_>>()).unwrap()
}
