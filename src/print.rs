pub fn execute() {
    println!("Hello from print.rs file");
    println!("print of a number: {}", 10);
    println!("{} is from {}", "I'm", "Brazil");
    println!(
        "This {2} an example {1} {0} arguments",
        "positional", "of", "is"
    );
    println!(
        "{word1} is an example of {word2} arguments",
        word1 = "This",
        word2 = "named"
    );
    println!("Binary: {:b}\tHexadecimal: {:x}\tOctal: {:o}", 10, 10, 10);
    println!(
        "With positional...\t\tBinary: {bin:b}\tHexadecimal: {hex:x}\tOctal: {oct:o}",
        hex = 15,
        bin = 2,
        oct = 8
    );
    println!("Debug print {:?}", (10, true, "yep yep yep"));
    println!("Basic math, like, 10 + 10 = {}", 10 + 10);
}
