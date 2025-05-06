use plotters::prelude::*;
use std::error::Error;
use csv::ReaderBuilder;
// use plotters::style::IntoTextStyle;

#[allow(dead_code)]
pub fn visualize_predictions_from_data(
    true_labels: &[i32],
    predicted_labels: &[i32],
    confidences: &[f64],
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut indexes = Vec::new();
    let mut is_correct = Vec::new();

    for i in 0..true_labels.len() {
        indexes.push(i as f64);
        is_correct.push(predicted_labels[i] == true_labels[i]);
    }

    // Gambar grafik
    let root = BitMapBackend::new(output_path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_index = indexes.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("Sebaran Confidence Prediksi ANN", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..(max_index as f64), 0f64..1.05)?;

    chart
        .configure_mesh()
        .x_desc("Index Data")
        .y_desc("Confidence")
        .draw()?;

    for i in 0..indexes.len() {
        let color = match predicted_labels[i] {
            0 => BLUE.mix(1.0),   // Normal
            1 => GREEN.mix(1.0),  // Maintenance
            2 => RED.mix(1.0),    // Fail
            _ => BLACK.mix(1.0),  // Tidak diketahui
        };

        let style = if is_correct[i] {
            ShapeStyle {
                color,
                filled: true,
                stroke_width: 1,
            }
        } else {
            ShapeStyle {
                color: BLACK.mix(0.4),
                filled: false,
                stroke_width: 2,
            }
        };

        chart.draw_series(std::iter::once(Circle::new(
            (indexes[i], confidences[i]),
            3,
            style,
        )))?;
    }

    // Tambahkan legenda
    root.draw(&Text::new(
        "📘 Biru: Normal    📗 Hijau: Maintenance    📕 Merah: Fail\n⭕ Outline: Salah prediksi",
        (30, 690),
        ("sans-serif", 20).into_font(),
    ))?;

    println!("✅ Visualisasi dari data berhasil disimpan di '{}'", output_path);
    Ok(())
}

