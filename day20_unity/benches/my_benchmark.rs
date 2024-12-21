use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion)
{
    c.bench_function("Part 1", |b| b.iter(|| day20_unity::run_part1()));
    //c.bench_function("Part 2", |b| b.iter(|| day20_unity::run_part2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);