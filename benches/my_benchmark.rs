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
    c.bench_function("min 4096", |b| {
        let mut rng = rand::thread_rng();

        let vals: Vec<f64> = (0..4096)
            .map(|_| rng.gen_range(0.0f64..1000.0f64))
            .collect();

        let slice = &vals[..];

        b.iter(|| find_minimum(black_box(slice)))
    });
}

criterion_group!(benches, bench_minimum);
criterion_main!(benches);
