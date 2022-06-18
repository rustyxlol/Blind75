fn max_sub_array(numbers: Vec<i32>) -> i32 {
    let mut max_sum = -9999;
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            let mut sum = 0;
            for k in i..j + 1 {
                sum += numbers[k];
            }
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }
    max_sum
}

fn max_sub_array_a2(numbers: Vec<i32>) -> i32 {
    let mut max_sum = -9999;
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in (i + 1)..numbers.len() {
            sum += numbers[j];
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }
    max_sum
}


fn max_sub_array_a3(numbers:Vec<i32>) -> i32 {
    let mut max_sum = numbers[0];
    let mut max_current = numbers[0];

    for i in 1..numbers.len() {
        max_current = std::cmp::max(numbers[i], max_current + numbers[i]);
        max_sum = std::cmp::max(max_sum, max_current);
    }
    max_sum
}

fn main() {
    let v1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let v2 = vec![5, 4, -1, 7, 8];
    println!("{}", max_sub_array(v1.clone()));
    println!("{}", max_sub_array(v2.clone()));
    println!("{}", max_sub_array_a3(v1));
    println!("{}", max_sub_array_a3(v2));
}
