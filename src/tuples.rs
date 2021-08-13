pub fn run() {
    let person: (&str, &str, i8) = ("James", "mass", 36);
    println!("{} and is from {} and is {} yrsold", person.0, person.1, person.2); 
}