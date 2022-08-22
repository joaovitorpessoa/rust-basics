/*
Primitive types ---
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays
*/
pub fn execute() {
    println!("Max value for an i32 {}", std::i32::MAX);

    // Default is "f64"
    let y = 2.5;

    let a1: char = 'a';
    let face: char = '\u{1F600}';

    println!("{:?}", (a1, face, y));
}
