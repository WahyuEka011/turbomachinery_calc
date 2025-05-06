use std::io::{self, Write};
use ndarray::Array1;
use plotters::prelude::*;



/// Fungsi untuk input manual dari terminal
#[allow(dead_code)]
pub fn get_user_input() -> Array1<f32> {
    let mut inputs = vec![];
    let param_names = ["Electricity (MW)", "RPM", "Pressure Steam (Bar)", "Vibrasi (mm/s)"];

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

#[allow(dead_code)]
pub fn read_user_input() -> Array1<f32> {
    fn prompt(msg: &str) -> f32 {
        println!("{}", msg);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        input.trim().parse::<f32>().unwrap_or(0.0)
    }

    let electricity = prompt("Masukkan Electricity (MW):");
    let rpm = prompt("Masukkan RPM:");
    let steam = prompt("Masukkan Pressure Steam (Bar):");
    let vibration = prompt("Masukkan Vibrasi (mm/s):");

    Array1::from(vec![electricity, rpm, steam, vibration])
}

#[allow(dead_code)]
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
            0 => &RED,      // Fail
            1 => &YELLOW,   // Maintenance
            2 => &GREEN,    // Normal
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

#[allow(dead_code)]
pub fn label_to_text(label: usize) -> &'static str {
    match label {
        0 => "Normal",
        1 => "Maintenance",
        2 => "Fail",
        _ => "Unknown",
    }
}

