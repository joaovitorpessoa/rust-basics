pub fn execute() {
    let mut hello = String::from("something");

    println!("Str: '{}' Length: {}", hello, hello.len());

    hello.push(' ');
    hello.push('n');
    hello.push('e');
    hello.push('w');

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("string created by explicit allocation of memory '{}'", s);
}
