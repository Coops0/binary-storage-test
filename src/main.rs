fn main() {
    // let before_generation = Instant::now();
    // let logs: Vec<PlayerLog> = (0..1_000_000)
    //     .into_par_iter()
    //     .map(|_| log_generator().build().unwrap())
    //     .collect();

    // println!(
    //     "! generated {} logs in {}",
    //     logs.len(),
    //     format_duration(before_generation.elapsed())
    // );

    // {
    //     // we let serde_json use builders to be more fair so it doesn't have to use the byte arrays
    //     let log_builders = logs
    //         .iter()
    //         .map(PlayerLogBuilder::from_log)
    //         .collect::<Result<Vec<PlayerLogBuilder>>>()
    //         .unwrap();

    //     let instant = Instant::now();

    //     let bytes = test_serde_json(&log_builders);

    //     println!(
    //         "serde_json: {}µs, {}",
    //         format_duration(instant.elapsed()),
    //         ByteSize(bytes as u64)
    //     );
    // }

    // {
    //     let instant = Instant::now();

    //     let bytes = test_postcard(&logs);

    //     println!(
    //         "postcard: {}, {}",
    //         format_duration(instant.elapsed()),
    //         ByteSize(bytes as u64)
    //     );
    // }

    // {
    //     let instant = Instant::now();

    //     let bytes = test_bincode(&logs);

    //     println!(
    //         "bincode: {}µs, {}",
    //         format_duration(instant.elapsed()),
    //         ByteSize(bytes as u64)
    //     );
    // }

    // {
    //     let instant = Instant::now();

    //     let bytes = test_our_serialization(&logs);

    //     println!(
    //         "our_serialization: {}µs, {}",
    //         format_duration(instant.elapsed()),
    //         ByteSize(bytes as u64)
    //     );
    // }

    // println!("all tests successful!");
}
