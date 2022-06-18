use std::collections::HashSet;
use std::iter::FromIterator;

fn contains_duplicate(numbers: Vec<i32>) -> bool {
    let resultant_set:HashSet<&i32> = HashSet::from_iter(numbers.iter());

    !(&resultant_set.len() == &numbers.len())
}

fn contains_duplicate_a2(numbers: Vec<i32>) -> bool {
    for i in 0..(numbers.len()) {
        for j in 0..(numbers.len()) {
            if i != j {
                if numbers[i] == numbers[j] {
                    return true;
                }
            }
        }
    }
    false
}

fn contains_duplicate_a3(numbers: Vec<i32>) -> bool {
    let mut sorted = numbers.clone();
    sorted.sort();

    for i in 0..(sorted.len() - 1) {
        if sorted[i] == sorted[i+1] {
            return true
        }
    }
    false
}


fn main() {
    let v1: Vec<i32> = vec![1,2,3,1];
    let v2: Vec<i32> = vec![1,2,3,4];
    let v3: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];

    println!("{}", contains_duplicate_a3(v1));
    println!("{}", contains_duplicate_a3(v2));
    println!("{}", contains_duplicate_a3(v3));
}
