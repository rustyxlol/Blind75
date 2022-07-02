fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
    for i in 1..text1.len() + 1 {
        for j in 1..text2.len() + 1 {
            if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                dp[i][j] = std::cmp::max(dp[i][j], 1 + dp[i - 1][j - 1]);
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[text1.len()][text2.len()]
}
fn main() {
    println!(
        "{}",
        longest_common_subsequence("abcde".to_string(), "ace".to_string(),)
    );
}
