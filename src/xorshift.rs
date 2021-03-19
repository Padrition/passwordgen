use std::time::UNIX_EPOCH;
use std::time::SystemTime;

pub fn get_rand(len : u128) -> u128{
    let now = SystemTime::now();
    let from_unix = now.duration_since(UNIX_EPOCH).expect("Congrats on time travel!");
    let seed = from_unix.as_micros();
    let x = seed;
    let x = x ^ seed << 13;
    let x = x ^ x >>17;
    let x = x ^ x <<5;
    let x = x % len;
    x as u128
}
