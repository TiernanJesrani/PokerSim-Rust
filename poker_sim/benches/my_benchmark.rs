use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use poker_sim::models::monte_model::MonteModel;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("monte_sim", |b| {
        b.iter(|| {
            let mut monte = MonteModel::new(1_000);
            black_box(monte.run_sim());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);