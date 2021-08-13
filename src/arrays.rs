use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,10];
    println!("{:?}", numbers[4]);

    let mut stuff: [i32; 2] = [54, 67];
    stuff[1] = 12;
    println!("{}", stuff[1]);

    // arrays are stack allocated and we can get how many bytes they're using:
    println!("Array occupies {} bytes on stack", mem::size_of_val(&stuff)); // size in bytes

    // get slices of array
    let slice: &[i32] = &numbers[0..2]; // pick start and end of slice
    println!("Slice: {:?}", slice);
}