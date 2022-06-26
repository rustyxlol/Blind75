fn three_sum(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums = numbers.clone();
    sorted_nums.sort();

    let mut results = vec![];

    for i in 0..sorted_nums.len() {
        for j in (i + 1)..sorted_nums.len() {
            for k in (j + 1)..sorted_nums.len() {
                if (sorted_nums[i] + sorted_nums[j] + sorted_nums[k]) == 0 {
                    let result = vec![sorted_nums[i], sorted_nums[j], sorted_nums[k]];
                    if results.contains(&result) {
                        continue;
                    }
                    results.push(result);
                }
            }
        }
    }

    results
}

fn three_sum_a2(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums = numbers.clone();
    sorted_nums.sort();

    let mut results = vec![];

    for i in 0..sorted_nums.len() {
        if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
            continue;
        }
        let curr_element = sorted_nums[i];
        let mut low = i + 1;
        let mut high = sorted_nums.len() - 1;

        while low < high {
            let sum = curr_element + sorted_nums[low] + sorted_nums[high];
            if sum == 0 {
                results.push([curr_element, sorted_nums[low], sorted_nums[high]].to_vec());
                low = low + 1;

                while sorted_nums[low] == sorted_nums[low - 1] && low < high {
                    low = low + 1;
                }
            } else if sum > 0 {
                high = high - 1;
            } else {
                low = low + 1;
            }
        }
    }

    results
}

fn main() {
    let v1 = vec![-1, 0, 1, 2, -1, -4];

    println!("{:?}", three_sum_a2(v1));
}
