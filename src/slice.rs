pub fn slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[0..3];
    println!("{:?}", slice);


    let slice = &a[1..3];
    println!("{:?}", slice);


    let slice = &a[2..5];
    println!("{:?}", slice);
}