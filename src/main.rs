mod fibonacci;

fn main() {
    for n in 1..11 {
        print!("{}th Fibonacci number: {}\n", n, fibonacci::get_fibonacci(n));
    }
}
