use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion)
{
    c.bench_function("Part 1&2", |b| b.iter(|| day7::get_results_program()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);