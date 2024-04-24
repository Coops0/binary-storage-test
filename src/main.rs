use std::{env, mem::size_of_val, time::Instant};

use anyhow::Result;
use binary_storage_test::{
    log_generator,
    player_log::{deserialize_vec, serialize_vec, PlayerLog, PlayerLogBuilder},
};
use bytesize::ByteSize;
use humantime::format_duration;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let before_generation = Instant::now();
    let logs: Vec<PlayerLog> = (0..100_000)
        .into_par_iter()
        .map(|_| log_generator().build().unwrap())
        .collect();

    println!(
        "! generated {} logs in {}, {}",
        logs.len(),
        format_duration(before_generation.elapsed()),
        ByteSize(size_of_val(&*logs) as u64)
    );

    {
        // we let serde_json use builders to be more fair so it doesn't have to use the byte arrays
        let log_builders = logs
            .iter()
            .map(PlayerLogBuilder::from_log)
            .collect::<Result<Vec<PlayerLogBuilder>>>()
            .unwrap();

        let instant = Instant::now();

        let serialized = serde_json::to_string(&log_builders).unwrap();
        let deserialized: Vec<PlayerLogBuilder> = serde_json::from_str(&serialized).unwrap();

        println!(
            "serde_json: {}µs, {}",
            format_duration(instant.elapsed()),
            ByteSize(serialized.bytes().len() as u64)
        );

        assert_eq!(log_builders, deserialized);
    }

    {
        let instant = Instant::now();

        let serialized = postcard::to_allocvec(&logs).unwrap();
        let deserialized: Vec<PlayerLog> = postcard::from_bytes(&serialized).unwrap();

        println!(
            "postcard: {}, {}",
            format_duration(instant.elapsed()),
            ByteSize(serialized.len() as u64)
        );

        assert_eq!(logs, deserialized);
    }

    {
        let instant = Instant::now();

        let serialized = bincode::serialize(&logs).unwrap();
        let deserialized: Vec<PlayerLog> = bincode::deserialize(&serialized).unwrap();

        println!(
            "bincode: {}µs, {}",
            format_duration(instant.elapsed()),
            ByteSize(serialized.len() as u64)
        );

        assert_eq!(logs, deserialized);
    }

    {
        let instant = Instant::now();

        let serialized = serialize_vec(&logs).unwrap();
        let deserialized: Vec<PlayerLog> = deserialize_vec(&serialized).unwrap();

        println!(
            "our_serialization: {}µs, {}",
            format_duration(instant.elapsed()),
            ByteSize(serialized.len() as u64)
        );

        assert_eq!(logs, deserialized);
    }

    println!("all tests successful!");
}
