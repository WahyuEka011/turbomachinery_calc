use std::io;
use plotters::prelude::*;
use plotters::style::IntoTextStyle;

mod svm;
mod knn;
mod lookup;

// Fungsi faktorial untuk perhitungan deret Taylor
fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}

// Implementasi sin(x) dengan deret Taylor
fn taylor_sin(x: f64, terms: u32) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let term = sign * x.powi(2 * n as i32 + 1) / factorial(2 * n + 1) as f64;
        result += term;
    }
    result
}

// Implementasi cos(x) dengan deret Taylor
fn taylor_cos(x: f64, terms: u32) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let term = sign * x.powi(2 * n as i32) / factorial(2 * n) as f64;
        result += term;
    }
    result
}

// Fungsi untuk membuat grafik
fn create_chart(angle_deg: f64, sin_taylor: f64, cos_taylor: f64, sin_lookup: f64, cos_lookup: f64) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output.png", (651, 321)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
        "Perbandingan Sin(x) dan Cos(x): Deret Taylor vs Lookup Table",
        ("sans-serif", 20).into_text_style(&root),
    )
        .build_cartesian_2d(0f64..360f64, -1.0f64..1.0f64)?;

    chart.configure_mesh().draw()?;

    let sin_taylor_data: Vec<(f64, f64)> = vec![(angle_deg, sin_taylor)];
    let cos_taylor_data: Vec<(f64, f64)> = vec![(angle_deg, cos_taylor)];
    let sin_lookup_data: Vec<(f64, f64)> = vec![(angle_deg, sin_lookup)];
    let cos_lookup_data: Vec<(f64, f64)> = vec![(angle_deg, cos_lookup)];

    // Plot sin dan cos dari deret Taylor
    chart.draw_series(LineSeries::new(sin_taylor_data, &RED))?
        .label("Sin (Taylor)")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));

    chart.draw_series(LineSeries::new(cos_taylor_data, &BLUE))?
        .label("Cos (Taylor)")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &BLUE));

    // Plot sin dan cos dari lookup table
    chart.draw_series(LineSeries::new(sin_lookup_data, &GREEN))?
        .label("Sin (Lookup Table)")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &GREEN));

    chart.draw_series(LineSeries::new(cos_lookup_data, &CYAN))?
        .label("Cos (Lookup Table)")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &CYAN));

    // Menambahkan legenda
    chart.configure_series_labels().position(SeriesLabelPosition::UpperLeft).draw()?;

    Ok(())
}

fn main() {
    let mut input = String::new();
    println!("Masukkan sudut dalam derajat:");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");

    let angle_deg: f64 = input.trim().parse().expect("Harap masukkan angka!");
    let angle_rad = angle_deg.to_radians(); // Konversi ke radian
    let terms = 10; // Jumlah suku Taylor yang digunakan

    // Hitung nilai dengan deret Taylor
    let sin_taylor = taylor_sin(angle_rad, terms);
    let cos_taylor = taylor_cos(angle_rad, terms);

    // Hitung nilai dengan lookup table
    let sin_lookup = lookup::lookup_sin(angle_deg);
    let cos_lookup = lookup::lookup_cos(angle_deg);

    // Output hasil
    println!("(Deret Taylor) sin({}°) ≈ {}", angle_deg, sin_taylor);
    println!("(Deret Taylor) cos({}°) ≈ {}", angle_deg, cos_taylor);
    println!("(Lookup Table) sin({}°) ≈ {}", angle_deg, sin_lookup);
    println!("(Lookup Table) cos({}°) ≈ {}", angle_deg, cos_lookup);

    // Buat grafik dan simpan sebagai output.png
    if let Err(e) = create_chart(angle_deg, sin_taylor, cos_taylor, sin_lookup, cos_lookup) {
        println!("Gagal membuat grafik: {}", e);
    }
}
