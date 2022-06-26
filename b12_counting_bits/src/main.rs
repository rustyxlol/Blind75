fn count_ones(n: i32) -> i32 {
    let mut count = 0;
    let mut num = n;

    while num > 0 {
        num = num & (num - 1);
        count += 1;
    }

    count
}

fn count_bits_a2(n: i32) -> Vec<i32> {
    let n = (n + 1) as usize;
    let mut results = vec![0; n];
    for i in 1..n {
        results[i] = results[i >> 1] + (i & 1) as i32;
    }
    results
}

fn count_bits(n: i32) -> Vec<i32> {
    let mut results = Vec::new();
    for i in 0..n + 1 {
        results.push(count_ones(i));
    }
    results
}
fn main() {
    println!("{:?}", count_bits(5));
    println!("{:?}", count_bits(2));
    println!("{:?}", count_bits_a2(2));
}
