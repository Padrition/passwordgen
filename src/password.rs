use super::xorshift;

static ASCII : [char; 69] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z', 
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
    '1','2','3','4',
    '5','6','7','8',
    '9','0',
    '+','-','_','?','!','$','/',
];

pub fn rand_pass(pass_len : u32)-> String{
    let mut  password = String::new();
    for i in 0..pass_len{
        password.push(ASCII[xorshift::get_rand(2)]);
    }
    password
}