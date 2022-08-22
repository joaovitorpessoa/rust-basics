// Like vectors of STL, they are resizable arrays

pub fn execute() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    numbers.push(6);
    numbers.push(7);
    numbers.push(8);

    numbers.pop();

    println!("Vector of numbers: {:?}", numbers);
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Without mutation of elements
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // With mutation of elements
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
