fn max_area(height: Vec<i32>) -> i32 {
    let mut maximum = 0;
    for i in 0..height.len() {
        for j in i+1..height.len() {
            let min_height = std::cmp::min(height[i], height[j]);
            let area = ((j-i) as i32) * min_height;
            maximum = std::cmp::max(maximum, area);
        }
    }
    maximum
}

fn max_area_a2(height: Vec<i32>) -> i32 {
    let mut maximum = 0;
    let mut left_ptr = 0;
    let mut right_ptr = height.len() - 1;
    while left_ptr < right_ptr {
        let min_height = std::cmp::min(height[left_ptr], height[right_ptr]);
        let width = (right_ptr - left_ptr) as i32;
        let area = min_height * width;
        
        if height[left_ptr] > height[right_ptr] {
            right_ptr -= 1;
        } else {
            left_ptr += 1;
        }
        
        maximum = std::cmp::max(maximum, area);
    }
    maximum
}
fn main() {
    let v1 = vec![1,8,6,2,5,4,8,3,7];
    println!("{}", max_area_a2(v1));
}
