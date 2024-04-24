use binary_storage_test::{player_log::*, *};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Serialization");

    group.bench_with_input("serde_json", &10_000, |b, &size| {
        b.iter_batched(
            || {
                (0..size)
                    .into_iter()
                    .map(|_| log_generator())
                    .collect::<Vec<PlayerLogBuilder>>()
            },
            |data| {
                let serialized = serde_json::to_string(&data).unwrap();
                let deserialized: Vec<PlayerLogBuilder> =
                    serde_json::from_str(&serialized).unwrap();
                assert_eq!(data, deserialized);
                serialized.len()
            },
            BatchSize::NumBatches(size),
        )
    });

    group.bench_with_input("postcard", &10_000, |b, &size| {
        b.iter_batched(
            || {
                (0..size)
                    .into_iter()
                    .map(|_| log_generator().build().unwrap())
                    .collect::<Vec<PlayerLog>>()
            },
            |data| {
                let serialized = postcard::to_allocvec(&data).unwrap();
                let deserialized: Vec<PlayerLog> = postcard::from_bytes(&serialized).unwrap();
                assert_eq!(data, deserialized);
                serialized.len()
            },
            BatchSize::NumBatches(size),
        )
    });

    group.bench_with_input("bincode", &10_000, |b, &size| {
        b.iter_batched(
            || {
                (0..size)
                    .into_iter()
                    .map(|_| log_generator().build().unwrap())
                    .collect::<Vec<PlayerLog>>()
            },
            |data| {
                let serialized = bincode::serialize(&data).unwrap();
                let deserialized: Vec<PlayerLog> = bincode::deserialize(&serialized).unwrap();
                assert_eq!(data, deserialized);
                serialized.len()
            },
            BatchSize::NumBatches(size),
        )
    });

    group.bench_with_input("our_serialization", &10_000, |b, &size| {
        b.iter_batched(
            || {
                (0..size)
                    .into_iter()
                    .map(|_| log_generator().build().unwrap())
                    .collect::<Vec<PlayerLog>>()
            },
            |data| {
                let serialized = serialize_vec(&data).unwrap();
                let deserialized = deserialize_vec(&serialized).unwrap();
                assert_eq!(data, deserialized);
                serialized.len()
            },
            BatchSize::NumBatches(size),
        )
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
