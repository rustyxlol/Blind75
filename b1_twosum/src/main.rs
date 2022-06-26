use std::collections::HashMap;

fn two_sum_a1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                results.push(i as i32);
                results.push(j as i32);
            }
        }
    }
    results
}

fn two_sum_a2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut results = HashMap::new();

    for i in 0..numbers.len() {
        match results.get(&(&target - &numbers[i])) {
            Some(&index) => return vec![index, i as i32],
            None => results.insert(numbers[i], i as i32),
        };
    }
    vec![]
}

fn main() {
    // [2,7,11,15], target = 9 
    // output: [0,1]

    // [3,2,4], target = 6 
    // output: [1,2]

    // [3,3], target = 6 
    // output: [0,1]

    let e1 = vec![2, 1, 3, 5];
    let e2 = vec![2,7,11,15];
    let e3 = vec![3,2,4];
    let e4 = vec![3,3];


    // println!("{:?}", two_sum_a1(e1, 5));
    println!("{:?}", two_sum_a2(e1, 5));
    
    // println!("{:?}", two_sum_a1(e2, 9));
    println!("{:?}", two_sum_a2(e2, 9));
    
    // println!("{:?}", two_sum_a1(e3, 6));
    println!("{:?}", two_sum_a2(e3, 6));
    
    // println!("{:?}", two_sum_a1(e4, 6));
    println!("{:?}", two_sum_a2(e4, 6));

}
