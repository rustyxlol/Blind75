fn reverse_bits(mut x: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..32 {
        result = result << 1;
        result = result + (x % 2);
        x = x >> 1;
    }
    result
}
fn main() {
    println!("{}", reverse_bits(0b00000010100101000001111010011100u32));
}
