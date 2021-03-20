use super::xorshift;

pub static ASCII : &[char] = &[
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

pub fn rand_pass(pass_len : u32, excluded : &[char])-> String{
    std::iter::repeat_with(|| ASCII[xorshift::get_rand(ASCII.len() as u128) as usize])
        .filter(|letter| !excluded.contains(letter))
        .take(pass_len as usize)
        .collect()
}