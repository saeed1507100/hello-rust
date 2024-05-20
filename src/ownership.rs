//string literals can't be mutable but String can be mutable
pub fn mutable_string(){
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, worild!", s1) // s1 is moved to s2 (s1 is not valid anymore)

    let s3= ownership_move(s2);
    //println!("{}, worild!", s2) // s2 is moved to ownership_move function (s2 is not valid anymore)

    println!("{}, worild!", s3) // s3 is valid here
}

pub fn ownership_move(s: String) -> String{
    println!("{}, worild!", s);
    return s;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn reference() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

pub fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}