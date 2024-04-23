use std::{iter, net::Ipv4Addr};

use player_log::{LogFlags, VERSIONS};
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use crate::player_log::PlayerLogBuilder;

pub mod player_log;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn rand_string(len: usize) -> String {
    let rng = &mut rand::thread_rng();
    iter::repeat_with(|| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .take(len)
        .collect()
}

fn rand_ip(rng: &mut ThreadRng) -> Ipv4Addr {
    Ipv4Addr::from([
        rng.gen_range(1..255),
        rng.gen_range(1..255),
        rng.gen_range(1..255),
        rng.gen_range(1..255),
    ])
}

pub fn log_generator() -> PlayerLogBuilder {
    let rng = &mut rand::thread_rng();
    let player_uuid = if rng.gen() {
        Some(uuid::Uuid::new_v4())
    } else {
        None
    };

    let mut flags = LogFlags::empty();
    if player_uuid.is_some() {
        flags.insert(LogFlags::IS_ONLINE);
    }

    if rng.gen() {
        flags.insert(LogFlags::PLAYER_AUTH);
    }

    PlayerLogBuilder {
        flags,
        player_uuid,
        player_name: rand_string(rng.gen_range(4..16)),
        player_ip: rand_ip(rng),
        server_ip: rand_ip(rng),
        server_port: rng.gen::<u16>(),
        server_domain: rand_string(rng.gen_range(4..255)),
        server_version: VERSIONS.entries().choose(rng).unwrap().0.to_string(),
    }
}
