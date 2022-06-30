fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut max_count: i32 = 0;
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
               dp[i] = dp[j] + 1; 
            }
        }
        max_count = std::cmp::max(dp[i], max_count);
    }
    max_count
}

fn main() {
    let v1 = vec![7, 7, 7, 7, 7, 7, 7];
    println!("{}", length_of_lis(v1));
}
