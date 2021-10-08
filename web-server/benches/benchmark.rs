use criterion::{
    criterion_group, criterion_main, Criterion, SamplingMode, Throughput,
};

// use web_server_lib::WebServer;

fn single_handler(c: &mut Criterion) {
    // Start the webserver in a new thread so we can benchmark it.
    // let _web_server = std::thread::spawn(|| WebServer::start_with_separate_handler());

    // Create a group for the benchmarks
    let mut single_handler = c.benchmark_group("Single Handler");
    single_handler.sampling_mode(SamplingMode::Flat);
    single_handler.throughput(Throughput::Elements(1));

    single_handler.bench_with_input("Adding Values", "add/1", |b, path| {
        b.iter(|| send_request(path))
    });

    single_handler.finish();
}

fn separate_handler(c: &mut Criterion) {
    // Start the webserver in a new thread so we can benchmark it.
    // let _web_server = std::thread::spawn(|| WebServer::start_with_separate_handler());

    // Create a group for the benchmarks
    let mut separate_handler = c.benchmark_group("Separate Handler");
    separate_handler.sampling_mode(SamplingMode::Flat);
    separate_handler.throughput(Throughput::Elements(1));

    separate_handler.bench_with_input("Adding Values", "add/1", |b, path| {
        b.iter(|| send_request(path))
    });

    separate_handler.finish();
}

fn send_request(path: &str) -> Result<(), String> {
    let url = format!("http://localhost:8080/{}", path);
    let _res =
        reqwest::blocking::get(url).map_err(|_| String::from("Failed to send get request"))?;
    //     .text()
    //     .map_err(|_| String::from("Failed to get text"))?;

    // println!("{}", res);

    Ok(())
}

criterion_group!(benches, single_handler, separate_handler);
criterion_main!(benches);
