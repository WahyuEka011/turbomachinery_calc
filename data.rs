use ndarray::{Array2, Array1, Axis};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

        if line.trim().is_empty() {
            eprintln!("Skipping empty line {}", i + 1); // Debugging line
            continue; // Skip empty lines
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        
        // Memeriksa jika jumlah elemen pada baris salah
        if parts.len() != 6 {
            eprintln!("Skipping line {}: Incorrect number of elements. Found: {}", i + 1, parts.len());
            continue;
        }

        // Mengambil fitur (kolom 0 hingga 4)
        let feat: Vec<f32> = parts[0..5]
            .iter()
            .map(|s| s.parse::<f32>().unwrap_or_else(|_| {
                eprintln!("Warning: Invalid feature value on line {}. Defaulting to 0.0", i + 1);
                0.0
            }))
            .collect();

        // Mengambil label (kolom ke-5: Failure Type)
        let label_str = parts[5].trim(); // Ambil label (Failure Type)
        let label = match label_str {
            "No Failure" => 0.0,       // Label "No Failure" menjadi 0.0
            "Power Failure" => 1.0,    // Label "Power Failure" menjadi 1.0
            "Overheat" => 2.0,         // Label "Overheat" menjadi 2.0
            _ => {
                eprintln!("Warning: Unknown label '{}' on line {}, skipping", label_str, i + 1);
                continue; // Skip invalid label
            }
        };

        features.push(feat);
        labels.push(label);
    }

    println!("Loaded {} data points", features.len());

    // Memeriksa jika data kosong setelah pemrosesan
    if features.is_empty() {
        return Err("Dataset kosong! Pastikan format CSV benar.".into());
    }

    // Mengonversi fitur menjadi Array2 dan label menjadi Array1
    let x = Array2::from_shape_vec((features.len(), 5), features.concat())?;
    let y = Array1::from_vec(labels);

    Ok((x, y))
}

pub fn normalize_dataset(x: &Array2<f32>) -> Array2<f32> {
    let mean = x.mean_axis(Axis(0)).unwrap();
    let std = x.std_axis(Axis(0), 0.0);

    // Jika std deviasi 0, set ke 1 untuk menghindari pembagian dengan 0
    let std_safe = std.mapv(|v| if v == 0.0 { 1.0 } else { v });

    let normalized_rows: Vec<Array1<f32>> = x
        .rows()
        .into_iter()
        .map(|row| (&row - &mean) / &std_safe)
        .collect();

    ndarray::stack(Axis(0), &normalized_rows.iter().map(|r| r.view()).collect::<Vec<_>>()).unwrap()
}
