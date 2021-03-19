use std::io;

mod xorshift;
mod password;

fn main() {

    let length : u32;

    loop{
        let mut pass_leng_str = String::new();

        println!("Enter the length of wannable password: ");

        io::stdin()
            .read_line(&mut pass_leng_str)
            .expect("Faild to read the line");

        let pass_leng_str = pass_leng_str.trim();

        match pass_leng_str.parse::<u32>(){
            Ok(i) => {
                length = i;
                break;
            },
            Err(..) => {
                println!("Not a valid integer!"); 
            },
        }
    }
    println!("{}", password::rand_pass(length));

}
