fn search_min(numbers: Vec<i32>) -> i32 {
    let mut left_ptr = 0;
    let mut right_ptr= numbers.len() - 1; 

    while left_ptr < right_ptr {
        let mid = (right_ptr + left_ptr) / 2;
        if numbers[mid] > numbers[right_ptr] {
            left_ptr = mid + 1;
        } else {
            right_ptr = mid;
        }
    }
    
    numbers[left_ptr]
}
fn main() {
    let v1 = vec![3,4,5,1,2];
    let v2 = vec![0,1,2,4,5,6,7];
    println!("{}", search_min(v1));
    println!("{}", search_min(v2));
}
