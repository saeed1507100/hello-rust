fn main() {
    let logical: bool = true;
    println!("{}", logical);
    let a_float: f64 = 1.0;  // Regular annotation
    println!("{}", a_float);
    let an_integer = 55i32;  // Suffix annotation
    println!("{}", an_integer);

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    println!("{}", inferred_type);
}