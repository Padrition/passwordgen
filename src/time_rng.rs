use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_rand(len: u128) -> u128 {
    let now = SystemTime::now();
    let from_unix = now
        .duration_since(UNIX_EPOCH)
        .expect("Congrats on time travel!");
    let seed = from_unix.as_nanos();
    let mut x = seed;
    x ^= seed << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x % len
}
