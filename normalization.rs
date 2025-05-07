use ndarray::{Array2, Axis};

pub struct Normalizer {
    min: Vec<f64>,
    max: Vec<f64>,
}

pub fn normalize_dataset(dataset: &Array2<f32>) -> Array2<f32> {
    // Cek dataset kosong
    if dataset.nrows() == 0 || dataset.ncols() == 0 {
        panic!("Dataset kosong saat normalisasi!");
    }

    let mean = dataset
        .mean_axis(Axis(0))
        .expect("Gagal menghitung rata-rata: dataset kosong");
    let std = dataset.std_axis(Axis(0), 0.0);

    let mean_broadcast = mean.broadcast(dataset.raw_dim())
        .expect("Broadcast mean gagal: dimensi tidak cocok");
    let std_broadcast = std.broadcast(dataset.raw_dim())
        .expect("Broadcast std gagal: dimensi tidak cocok");

    let normalized = dataset - &mean_broadcast;
    &normalized / &std_broadcast
}

impl Normalizer {
    pub fn fit(data: &Array2<f64>) -> Self {
        if data.nrows() == 0 || data.ncols() == 0 {
            panic!("Data kosong saat fit normalizer");
        }

        let mut min = vec![];
        let mut max = vec![];

        for axis in data.axis_iter(Axis(1)) {
            let col_min = axis.fold(f64::INFINITY, |a, &b| a.min(b));
            let col_max = axis.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            min.push(col_min);
            max.push(col_max);
        }

        Self { min, max }
    }

    pub fn transform(&self, data: &Array2<f64>) -> Array2<f64> {
        if data.ncols() != self.min.len() {
            panic!("Jumlah kolom tidak cocok antara data dan normalizer");
        }

        let mut normalized = data.clone();

        for (i, mut col) in normalized.axis_iter_mut(Axis(1)).enumerate() {
            let min = self.min[i];
            let max = self.max[i];
            let range = max - min;

            if range != 0.0 {
                for elem in col.iter_mut() {
                    *elem = (*elem - min) / range;
                }
            }
        }

        normalized
    }

    pub fn fit_transform(data: &Array2<f64>) -> (Self, Array2<f64>) {
        let normalizer = Self::fit(data);
        let transformed = normalizer.transform(data);
        (normalizer, transformed)
    }
}
