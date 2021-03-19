use std::io;

mod xorshift;
mod password;

fn main() {

    let length : u32;
    let mut chars_to_exclude : Vec<char> = vec![];

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
    //todo: make it possible to exclude characters
    //ask user a string of characters he would like to exdlude
    println!("Enter the characters you want to exclude: ");

    let mut exclude = String::new();

    io::stdin()
        .read_line(&mut exclude)
        .expect("Faild to read the line");
    
    for i in 0..exclude.trim().len(){
        let c = exclude.as_bytes()[i] as char;
        chars_to_exclude.push(c);

    }
    println!("{:?}", chars_to_exclude);

    //iterate random characters and do not include them into password if one matchs ones that excluded
    
    println!("{}", password::rand_pass(length));

}
