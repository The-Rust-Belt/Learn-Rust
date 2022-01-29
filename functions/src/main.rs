fn main() {
    println!("Hello, world!");
}

fn print_sum(a: i8, b: i8) {
    println!("Sum is: {}", a + b);
}

// Without the return keyword, only the last expression returns.
fn plus_one(a: i32) -> i32 {
    a + 1
    // Check it out: you don't need the semicolon for the last line in a Rust function.
    // However this means that the final line (without a semicolon) is the return value.
    // "return a + 1"
}

// With the return keyword
fn plus_two(a: i32) -> i32 {
    return a + 2;
    // In Rust, the return keyword at the end is actually bad practice.
    // You should end your functions with a semicolon-less line instead.
    // Only use return keyword for conditionals or where absolutely necessary.
}

// Function Pointers - Usage as a Data Type:
// This is crazy: you can declare a variable to be a function!
fn pointers() {
    // Without type declarations
    let p1 = plus_one;
    let x = p1(5); // 6

    // With type declarations
    let p1: fn(i32) -> i32 = plus_one;
    let x = p1(5); // 6
}