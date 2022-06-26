fn hamming_weight_a1(n: u32) -> i32 {
    n.count_ones() as i32
}

fn hamming_weight_a2(mut n: u32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n % 2;
        n = n >> 1;
    }
    count as i32
}


fn main() {
    println!("{}", hamming_weight_a1(0b00000000000000000000000000001011u32));
    println!("{}", hamming_weight_a2(0b00000000000000000000000000001011u32));
}
