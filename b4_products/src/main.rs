fn product_except_self_a1(numbers: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    
    for i in 0..numbers.len() {
        let mut product: i32 = 1;
        for j in 0..numbers.len() {
            if i != j {
                product = product * numbers[j];
            }
        }
        results.push(product);
    }
    results
}

fn product_except_self_a3(numbers: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let mut pre = vec![1; numbers.len()];
    let mut post = vec![1; numbers.len()];

    for i in 1..numbers.len() {
        pre[i] = (numbers[i-1] * pre[i-1]);
    }

    for i in (0..numbers.len() - 1).rev() {
        post[i] = numbers[i+1] * post[i+1];
    }

    for i in 0..numbers.len() {
        results.push(pre[i] * post[i]);
    }
    results
}

fn product_except_self_a4(numbers: Vec<i32>) -> Vec<i32> {
    let mut pre = vec![1; numbers.len()];
    let mut post_op: i32 = 1;

    for i in 1..numbers.len() {
        pre[i] = (numbers[i-1] * pre[i-1]);
    }

    for i in (0..numbers.len()).rev() {
        pre[i] = post_op * pre[i];
        post_op *= numbers[i];
    }

    pre
}

fn main() {
    let v1 = vec![1,2,3,4];
    let v2 = vec![-1,1,0,-3,3];
    println!("{:?}", product_except_self_a4(v1));
    println!("{:?}", product_except_self_a4(v2));
}
