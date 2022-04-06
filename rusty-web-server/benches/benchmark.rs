use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode, Throughput};
use rayon::prelude::*;

macro_rules! bench {
    ($feature: tt, $group: ident) => {{
        $group.bench_function($feature, |b| {
            let request_type = rand::random::<bool>();
            if request_type {
                b.iter(|| send_request("get".to_owned()));
            } else {
                let value = rand::random::<i32>();
                b.iter(|| send_request("/add/{value}".to_owned()));
            }
        });
    }};
}

const PARALLEL_CONNECTIONS: u64 = 10;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rusty Web Server Benchmarks");
    group.throughput(Throughput::Elements(PARALLEL_CONNECTIONS));
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(std::time::Duration::from_secs(60));
    group.sample_size(1_000);
    group.warm_up_time(std::time::Duration::from_secs(5));

    if cfg!(feature = "single_threaded") {
        bench!("Single Threaded", group);
    }

    if cfg!(feature = "original") {
        bench!("Original", group);
    }

    if cfg!(feature = "standard") {
        bench!("Standard", group);
    }

    if cfg!(feature = "macro") {
        bench!("Macro", group);
    }

    // let tree = rusty_tree::standard_tree::RustyTree::new();
    // group.bench_function("Standard Tree", |b| {
    //     let key = rand::random::<i64>();
    //     let value = rand::random::<i64>();
    //     b.iter(|| tree.insert(black_box(key), black_box(value)));
    // });

    // let tree = rusty_tree::macro_tree::RustyTree::new();
    // group.bench_function("Macro Tree", |b| {
    //     let key = rand::random::<i64>();
    //     let value = rand::random::<i64>();
    //     b.iter(|| tree.insert(black_box(key), black_box(value)));
    // });

    // let tree = rusty_tree::mutex_tree::RustyTree::new();
    // group.bench_function("Mutex Tree", |b| {
    //     let key = rand::random::<i64>();
    //     let value = rand::random::<i64>();
    //     b.iter(|| tree.insert(black_box(key), black_box(value)));
    // });

    group.finish();
}

pub fn send_request(path: String) {
    (0..PARALLEL_CONNECTIONS).into_par_iter().for_each(|_| {
        reqwest::blocking::get(format!("localhost:8080/{path}"))
            .map(|r| r.status().is_success())
            .unwrap_or(false);
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
