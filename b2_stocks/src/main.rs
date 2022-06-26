fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    let mut minimum: i32 = prices[0];

    for current_price in prices.iter().skip(1) {
        if current_price < &minimum {
            minimum = *current_price
        }

        if current_price - minimum > profit {
            profit = current_price - minimum;
        }
    }
    profit
}

fn max_profit_a2(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;
    let mut left_ptr = 0;
    let mut right_ptr = 1;

    while right_ptr <= prices.len() - 1 {
        if prices[left_ptr] < prices[right_ptr] {
            let profit = prices[right_ptr] - prices[left_ptr];

            if profit > max_profit {
                max_profit = profit;
            }
        } else {
            left_ptr = right_ptr;
        }
        right_ptr += 1
    }
    max_profit
}

fn main() {
    let vec_1 = vec![7, 1, 5, 3, 6, 4]; // 5
    let vec_2 = vec![7, 6, 4, 3, 1]; // 0

    println!("{}", max_profit(vec_1));
    println!("{}", max_profit_a2(vec_2));
}
