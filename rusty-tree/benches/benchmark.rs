use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, SamplingMode};

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let mut group = c.benchmark_group("RustyTree Benchmarks");
    group.throughput(Throughput::Elements(1));
    group.measurement_time(std::time::Duration::from_secs(90));
    group.sampling_mode(SamplingMode::Linear);
    group.sample_size(1_000);
    group.warm_up_time(std::time::Duration::from_secs(5));

    let tree = rusty_tree::original_tree::RustyTree::new();
    group.bench_function("Original Tree", |b| {
        let key = rand::random::<i64>();
        let value = rand::random::<i64>();
        b.iter(|| tree.insert(black_box(key), black_box(value)));
    });

    let tree = rusty_tree::standard_tree::RustyTree::new();
    group.bench_function("Standard Tree", |b| {
        let key = rand::random::<i64>();
        let value = rand::random::<i64>();
        b.iter(|| tree.insert(black_box(key), black_box(value)));
    });

    let tree = rusty_tree::macro_tree::RustyTree::new();
    group.bench_function("Macro Tree", |b| {
        let key = rand::random::<i64>();
        let value = rand::random::<i64>();
        b.iter(|| tree.insert(black_box(key), black_box(value)));
    });

    let tree = rusty_tree::mutex_tree::RustyTree::new();
    group.bench_function("Mutex Tree", |b| {
        let key = rand::random::<i64>();
        let value = rand::random::<i64>();
        b.iter(|| tree.insert(black_box(key), black_box(value)));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
