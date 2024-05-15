pub fn get_fibonacci(n: i32) -> i32 {
    let mut n1 = 0;
    let mut n2 = 1;
    let mut sum = 0;
    if n == 1 {
        return n1
    } else if n == 2 {
        return n2
    } else {
        for _ in 0..n-2 {
            sum = n1 + n2;
            n1 = n2;
            n2 = sum;
        }
    }
    
    sum
}