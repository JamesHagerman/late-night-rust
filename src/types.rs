pub fn run() {
    let x = 1; // i32
    let y = 2.5; // i64
    let d: i64 = 4654654654;
    println!("max 32{}", std::i32::MAX);
    println!("max 64 {}", std::i64::MAX);

    let active = true; // inferred bool
    println!("{:?}", (x, y, d, active));

    let is_greater = 10 > 5; // inferred bool
    println!("{}", is_greater);

    // char
    let char1 = 'a';
    println!("{:?}", char1);
}