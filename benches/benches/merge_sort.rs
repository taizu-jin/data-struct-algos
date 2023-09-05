use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{self, distributions::Uniform, Rng};

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort");
    group.sample_size(50);

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 100_000);

        let array: Vec<usize> = (0..*size).map(|_| rng.sample(range)).collect();

        group.bench_function(BenchmarkId::new("idiomic", size), |b| {
            b.iter_batched(
                || array.clone(),
                |mut v| merge_sort::idiomatic::sort(&mut v),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("book", size), |b| {
            b.iter_batched(
                || array.clone(),
                |mut v| merge_sort::book::sort(&mut v, 0, size - 1),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
