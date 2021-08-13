// Two types of strings
// primitive str = immutable fixed length string somewhere in memory
// String = growable heap allocated data structure that can be modified

pub fn run() {
    let some_string = "Hello"; // primitive string
    let mut growable_string = String::from("Some mutable string");
    println!("A string: {} and mutable string: {}", some_string, growable_string);

    growable_string.push(':'); // add a char

    println!("A string: {}", growable_string);

    growable_string.push_str(" with extra stuff!!");

    println!("{}", growable_string);

    println!("Capacity {}", growable_string.capacity());

    for word in growable_string.split_ascii_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());

    println!("{}", s);

}