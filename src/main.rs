use std::io;
fn main() {
    //(1)ask for the length 
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
                println!("{} is your password", i);
                length = i;
                break;
            },
            Err(..) => {
                println!("Not a valid integer!"); 
            },
        }
    }
    println!("{}", length);
}
