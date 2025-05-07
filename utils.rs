use std::io::{self, Write};
use ndarray::Array1;
use plotters::prelude::*;

/// Fungsi untuk input manual dari terminal (dengan 5 parameter)
pub fn get_user_input() -> Array1<f32> {
    let mut inputs = vec![];
    let param_names = ["Surface Temperature", "Steam Temperature", "RPM", "Torque", "Vibrasi"];

    for name in param_names.iter() {
        loop {
            print!("Masukkan {}: ", name);
            io::stdout().flush().unwrap();

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            match buffer.trim().parse::<f32>() {
                Ok(value) => {
                    inputs.push(value);
                    break;
                }
                Err(_) => {
                    println!("⚠️  Input tidak valid. Masukkan angka (misalnya: 1450.0)");
                }
            }
        }
    }

    Array1::from(inputs)
}

pub fn read_user_input() -> Array1<f32> {
    fn prompt(msg: &str) -> f32 {
        println!("{}", msg);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        input.trim().parse::<f32>().unwrap_or(0.0)
    }

    let surface_temp = prompt("Masukkan Surface Temperature (C):");
    let steam = prompt("Masukkan Steam Temperature (C):");
    let rpm = prompt("Masukkan RPM:");
    let torque = prompt("Masukkan Torque (N/m):");
    let vibration = prompt("Masukkan Vibration:");

    Array1::from(vec![surface_temp, steam, rpm, torque, vibration])
}

pub fn plot_prediction_confidence(confidences: &[(usize, f32, usize)]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output/prediction_scatter.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_confidence = 1.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sebaran Confidence Hasil Prediksi", ("sans-serif", 30))
        .margin(30)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..confidences.len(), 0f32..max_confidence)?;

    chart.configure_mesh()
        .x_desc("Index Sampel")
        .y_desc("Confidence")
        .draw()?;

    for (i, conf, label) in confidences {
        let color = match label {
            0 => &RED,      // No failure
            1 => &YELLOW,   // Power failure
            2 => &GREEN,    // Overheat
            _ => &BLACK,
        };

        chart.draw_series(PointSeries::of_element(
            vec![(*i, *conf)],
            5,
            color,
            &|c, s, st| return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;
    }

    println!("📊 Scatter plot disimpan di output/prediction_scatter.png");
    Ok(())
}

/// Fungsi untuk mengubah label ke teks sesuai dengan kategori baru
#[allow(dead_code)]
pub fn label_to_text(label: usize) -> &'static str {
    match label {
        0 => "No Failure",      // No failure
        1 => "Power Failure",   // Power failure
        2 => "Overheat",        // Overheat
        _ => "Unknown",         // Default case for unknown labels
    }
}
