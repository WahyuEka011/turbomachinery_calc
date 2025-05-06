mod lookup;
pub mod knn;
mod ann; 

use plotters::prelude::*;
// use std::path::Path;
use std::ffi::CString;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn get_sin_from_lookup(angle: f64) -> f64 {
    lookup::lookup_sin(angle)
}

#[no_mangle]
pub extern "C" fn get_cos_from_lookup(angle: f64) -> f64 {
    lookup::lookup_cos(angle)
}

#[no_mangle]
pub extern "C" fn draw_plot() {
    let root_area = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Sin and Cos", ("sans-serif", 30))
        .build_cartesian_2d(-3.14..3.14, -1.5..1.5)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-314..=314).map(|x| (x as f64 / 100.0, (x as f64 / 100.0).sin())),
            &RED,
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-314..=314).map(|x| (x as f64 / 100.0, (x as f64 / 100.0).cos())),
            &BLUE,
        ))
        .unwrap();
}

#[no_mangle]
pub extern "C" fn run_ann_from_qt(e: f64, r: f64, p: f64, v: f64) -> *const c_char {
    // Logika ANN kamu di sini
    let result = "Normal"; // atau "Maintenance", "Fail"

    // Ubah ke CString dan return pointer-nya
    CString::new(result).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() { return }
    unsafe {
        CString::from_raw(s);
    }
}