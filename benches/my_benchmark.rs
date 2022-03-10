use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

pub fn find_minimum(values: &[f64]) -> f64 {
    let mut min = f64::INFINITY;

    for &num in values {
        if num < min {
            min = num
        }
    }

    min
}

pub fn bench_minimum(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let vals: Vec<f64> = (0..4096)
        .map(|_| rng.gen_range(0.0f64..1000.0f64))
        .collect();

    let slice = &vals[..];

    c.bench_function("min 4096", |b| b.iter(|| find_minimum(black_box(slice))));
}

use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static TRIG_TABLE: Lazy<Mutex<HashMap<usize, (Vec<f64>, Vec<f64>)>>> = Lazy::new(|| {
    let m = HashMap::new();

    Mutex::new(m)
});

fn populate_trig_table(n: usize) {
    let mut map = TRIG_TABLE
        .lock()
        .expect("grabbed a lock on the trig tables");

    if map.contains_key(&n) {
        return;
    }

    let mut cos_table = Vec::with_capacity(n / 2);
    let mut sin_table = Vec::with_capacity(n / 2);

    for i in 0..(n / 2) {
        cos_table.push(((2.0 * std::f64::consts::PI * i as f64) / n as f64).cos());
        sin_table.push(((2.0 * std::f64::consts::PI * i as f64) / n as f64).sin());
    }

    map.insert(n, (cos_table, sin_table));
}

fn reverse_bits(val: usize, width: usize) -> usize {
    let mut result: usize = 0;
    let mut val = val;
    for _ in 0..width {
        result = (result << 1) | (val & 1);
        val = val >> 1;
    }
    result
}

pub fn transform_radix2(real: &mut [f64], imag: &mut [f64]) {
    let n = real.len();
    assert_eq!(real.len(), imag.len(), "Mismatched lengths");

    if n == 1 {
        // Trivial transform
        return;
    }

    let mut levels = 100000;
    for i in 0..32 {
        if 1 << i == n {
            levels = i; // equal to log2(n)
        }
    }

    assert_ne!(levels, 100000, "Length is not a power of 2");

    // Ensure there's something in the trig table
    populate_trig_table(n);

    let map = TRIG_TABLE
        .lock()
        .expect("grabbed a lock on the trig tables");

    let (cos_table, sin_table) = map.get(&n).expect("found a value for n");

    for i in 0..n {
        let j = reverse_bits(i, levels);
        if j > i {
            let mut temp = real[i];
            real[i] = real[j];
            real[j] = temp;
            temp = imag[i];
            imag[i] = imag[j];
            imag[j] = temp;
        }
    }

    // Cooley-Tukey decimation-in-time radix-2 FFT
    let mut size = 2;
    while size <= n {
        let half_size = size / 2;
        let table_step = n / size;

        for i in (0..n).step_by(size) {
            let mut k = 0;
            for j in i..(i + half_size) {
                let l = j + half_size;

                let tpre = real[l] * cos_table[k] + imag[l] * sin_table[k];
                let tpim = -real[l] * sin_table[k] + imag[l] * cos_table[k];
                real[l] = real[j] - tpre;
                imag[l] = imag[j] - tpim;
                real[j] += tpre;
                imag[j] += tpim;

                k += table_step;
            }
        }

        size *= 2;
    }
}

pub fn bench_fft(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut real: Vec<f64> = (0..4096)
        .map(|_| rng.gen_range(0.0f64..1000.0f64))
        .collect();
    let real = &mut real[..];

    let mut imag: Vec<f64> = vec![0.0; 4096];
    let imag = &mut imag[..];

    c.bench_function("fft 4096", |b| {
        b.iter(|| transform_radix2(black_box(real), black_box(imag)))
    });
}

criterion_group!(benches, bench_minimum, bench_fft);
criterion_main!(benches);
