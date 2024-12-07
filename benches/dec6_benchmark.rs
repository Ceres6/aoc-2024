use criterion::{criterion_group, criterion_main, Criterion};
#[path = "../src/dec6/mod.rs"]
mod dec6;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count loops", |b| b.iter(|| dec6::count_possible_loops()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);