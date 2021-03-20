use std::io;

mod password;
mod xorshift;

fn main() {
    let length: u32 = loop {
        let mut pass_leng_str = String::new();

        println!("Enter the length of wannable password: ");

        io::stdin()
            .read_line(&mut pass_leng_str)
            .expect("Faild to read the line");

        match pass_leng_str.trim().parse::<u32>() {
            Ok(i) => break i,
            Err(..) => {
                println!("Not a valid integer!");
            }
        }
    };

    let exclude: Vec<char> = {
        let mut excl_chars = String::new();

        println!("Write all characters you would like to exclude: ");

        io::stdin()
            .read_line(&mut excl_chars)
            .expect("Faild to read!");

        excl_chars.trim().chars().collect::<Vec<_>>()
    };

    println!("{}", password::rand_pass(length, &exclude));
}
