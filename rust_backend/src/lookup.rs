// Lookup table untuk sine dan cosine dalam rentang 0° - 360° dengan step 1°
use once_cell::sync::Lazy;

static SIN_TABLE: Lazy<Vec<f64>> = Lazy::new(|| {
    (0..=360).map(|i| (i as f64).to_radians().sin()).collect()
});

static COS_TABLE: Lazy<Vec<f64>> = Lazy::new(|| {
    (0..=360).map(|i| (i as f64).to_radians().cos()).collect()
});

// Fungsi untuk mengambil nilai dari lookup table
pub fn lookup_sin(angle: f64) -> f64 {
    let normalized_angle = ((angle % 360.0) + 360.0) % 360.0; // Normalisasi sudut
    SIN_TABLE[normalized_angle as usize]
}

pub fn lookup_cos(angle: f64) -> f64 {
    let normalized_angle = ((angle % 360.0) + 360.0) % 360.0; // Normalisasi sudut
    COS_TABLE[normalized_angle as usize]
}