mod fibonacci;

fn main() {
    let num = 20;
    let sum = fibonacci::odd_fibonacci_sum(num);
    println!("{sum}");
}
