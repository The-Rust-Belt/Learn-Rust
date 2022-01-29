// This is how you might usually do it
fn main() {
    let x = 2;
    println!("{}", get_square_value(x));
}

fn get_square_value(i: i32) -> i32 {
    i * i
}

// With type declarations of input and return types
fn with_type_dec() {
    let x = 2;
    let square = |i: i32| -> i32 { // Input parameters are passed inside | | and expression body is wrapped within { }
        i * i
    };
    println!("{}", square(x));
}

// Without type declarations of input and return types
fn without_type_dec() {
    let x = 2;
    let square = |i| i * i; // { } are optional for single-lined closures
    println!("{}", square(x));
}

// With type declarations, creating and calling together
fn with_type_dec_calling() {
    let x = 2;
    let square = |i: i32| -> i32 { i * i }(x); // { } are mandatory while creating and calling same time
    println!("{}", square);
}

// Without type declarations, creating and calling together
fn without_type_dec_calling() {
    let x = 2;
    let square = |i| { i * i }(x); // The return type is mandatory
    println!("{}", square);
}
