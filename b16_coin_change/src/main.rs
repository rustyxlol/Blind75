fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;

    for i in 1..(amount + 1) {
        for coin in &coins {
            if (i - coin) >= 0 {
                dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coin) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] != amount + 1 {
        dp[amount as usize]
    } else {
        -1
    }
}
fn main() {
    let v1 = vec![1, 2, 5];
    println!("{}", coin_change(v1, 11));
}
