fn max_psubarray(numbers: Vec<i32>) -> i32 {
    let mut max_product = numbers[0];
    for i in 0..numbers.len() {
        let mut product = 1;
        for j in i..numbers.len() {
            product *= numbers[j];
            if product > max_product {
                max_product = product;
            }
        }
    }
    max_product
}

fn max_psubarray_a2(numbers: Vec<i32>) -> i32 {
    let mut cur_max = numbers[0];
    let mut cur_min = numbers[0];
    let mut global_max = numbers[0];

    for i in 1..numbers.len() {
        if numbers[i] > 0 {
            cur_max = std::cmp::max(numbers[i] * cur_max, numbers[i]);
            cur_min = std::cmp::min(numbers[i] * cur_min, numbers[i]);
        } else if numbers[i] < 0 {
            let tmp_cur_max = cur_max;
            cur_max = std::cmp::max(numbers[i] * cur_min, numbers[i]);
            cur_min = std::cmp::min(numbers[i] * tmp_cur_max, numbers[i]);
        } else {
            cur_max = 0;
            cur_min = 0;
        }
        global_max = std::cmp::max(global_max, cur_max);
    }
    global_max
}

fn main() {
    let v1 = vec![2, 3, -2, 4];
    let v2 = vec![-2, 0, -1];

    println!("{}", max_psubarray_a2(v1));
    println!("{}", max_psubarray_a2(v2));
}
