// Like in C, arrays are FIXED list with elements of same data type

pub fn execute() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 20;

    println!("Array of numbers: {:?}", numbers);
    println!("Length: {}", numbers.len());
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice of numbers: {:?}", slice);
}
