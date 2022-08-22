pub fn execute() {
    let _sum: i32 = sum_ints(10, 5);

    let log_sum = |sum: i32| println!("Sum of 10 and 5 are {}", sum);

    log_sum(_sum);
}

fn sum_ints(x: i32, y: i32) -> i32 {
    let sum = x + y;
    return sum; // can be write without semicolon to omit the return keyword
}
