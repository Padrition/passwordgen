use super::time_rng;
use std::env;

pub struct Config {
    pub len : u32,
    pub excluded : Vec<char>,
}
impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();

        let len = match args.next(){
            Some(arg) => arg.trim().parse::<u32>().expect("Not a valid length"),
            None => return Err("Password length wasn't specified")
        };

        let excluded = match args.next(){
            Some(arg) => {
                arg.trim().chars().collect::<Vec<_>>()
            },
            None => return Err("Excluded characters was not specified")
        };

        Ok(Config{len, excluded})
    }
}
pub static ALLOWED_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5',
    '6', '7', '8', '9', '0', '+', '-', '_', '?', '!', '$', '/',
];

pub fn rand_pass(config : Config) -> String {
    std::iter::repeat_with(|| ALLOWED_CHARS[time_rng::get_rand(ALLOWED_CHARS.len() as u128) as usize])
        .filter(|letter| !config.excluded.contains(letter))
        .take(config.len as usize)
        .collect()
}
