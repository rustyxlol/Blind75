fn missing_number(numbers: &Vec<i32>) -> i32 {
    let num_len = numbers.len() ;
    let total_sum = (num_len * (num_len + 1))/2;
    let mut num_sum = 0;

    for i in numbers {
        num_sum += i;
    }
    total_sum as i32 - num_sum 
    
}

fn missing_number_a2(numbers: Vec<i32>) -> i32 {
    let mut result = numbers.len() as i32;

    for i in 0..numbers.len() {
        result = result ^ i as i32;
        result = result ^ numbers[i];
    }

    result
}
fn main() {
    let v = vec![9,6,4,2,3,5,7,0,1];
    println!("{}", missing_number(&v));
    println!("{}", missing_number_a2(v));
}
