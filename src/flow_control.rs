pub fn flow_control(){
    let condition = false;
    let number = if condition {4} else {5};

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

pub fn loop_example(){
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}