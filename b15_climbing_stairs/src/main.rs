// public static int Fibonacci(int N)
// {
//     double sqrt5 = Math.Sqrt(5);
//     double phi = (1 + sqrt5) / 2.0;
//     double fn = (Math.Pow(phi, N) - Math.Pow(1 - phi, N)) / sqrt5;
//     return (int)fn;
// }

fn climb_stairs(mut n: i32) -> i32 {
    n = n + 1;
    let sqrt_five = (5 as f64).sqrt();
    let phi = (1.0 + sqrt_five) / 2.0;
    let f_n = (f64::powf(phi, n.into()) - f64::powf(1.0 - phi, n.into())) / sqrt_five;

    println!("{}", f_n as i32);
    f_n as i32
}
fn main() {
    climb_stairs(2);
    climb_stairs(3);
    climb_stairs(4);
    climb_stairs(5);
    climb_stairs(6);
    climb_stairs(7);
    climb_stairs(8);
    climb_stairs(9);
}
