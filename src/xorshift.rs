use std::time::UNIX_EPOCH;
use std::time::SystemTime;

pub fn get_rand() -> u32{
    let now = SystemTime::now();
    let from_unix = now.duration_since(UNIX_EPOCH).expect("Congrats on time travel!");
    let seed = from_unix.as_micros();
    let x = seed;
    let x = x ^ seed << 13;
    let x = x ^ x >>17;
    let x = x ^ x <<5;
    return_last_digits_from_u128(x, 2)
}

fn return_last_digits_from_u128 (x : u128 ,len : usize) -> u32{
    let s = x.to_string();
    let slice = &s[s.len()-len..s.len()];
    let str = String::from(slice);
    let x : u32= str.parse().unwrap();
    x
}