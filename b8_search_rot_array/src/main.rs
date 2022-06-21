fn search_target(numbers: Vec<i32>, target: i32) -> i32 {
    let mut left_ptr = 0;
    let mut right_ptr = numbers.len() - 1;

    while left_ptr <= right_ptr {
        let mid_ptr = (right_ptr + left_ptr) / 2;

        if numbers[mid_ptr] == target {
            return mid_ptr as i32;
        }

        if numbers[mid_ptr] >= numbers[left_ptr] {
            // Check if target between low and mid
            if numbers[left_ptr] <= target && target <= numbers[mid_ptr] {
                right_ptr = mid_ptr;
            } else {
                left_ptr = mid_ptr + 1;
            }
        } else {
            // Check if target between mid and high
            if numbers[mid_ptr] <= target && target <= numbers[right_ptr] {
                left_ptr = mid_ptr + 1;
            } else {
                right_ptr = mid_ptr;
            }
        }
    }
    -1
}
fn main() {
    let v1 = vec![4, 5, 6, 7, 0, 1, 2];
    let v2 = vec![4, 5, 6, 7, 0, 1, 2];
    let v3 = vec![1, 3];
    println!("{}", search_target(v1, 0));
    println!("{}", search_target(v2, 3));
    println!("{}", search_target(v3, 3));
}
