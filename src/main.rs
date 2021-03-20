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

        match pass_leng_str.trim().parse::<u32>(){
            Ok(i) => {
                length = i;
                break;
            },
            Err(..) => {
                println!("Not a valid integer!"); 
            },
        }
    }

    let mut excl_chars = String::new();
    let exclude : Vec<char> ={
        io::stdin()
            .read_line(&mut excl_chars)
            .expect("Faild to read!");
        
        excl_chars.trim().chars().collect::<Vec<_>>()
    };

    println!("{}", password::rand_pass(length, &exclude));

}
