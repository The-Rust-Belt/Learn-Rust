fn main() {
    println!("{}, {}!", "Hello", "world"); // Hello, world!
    println!("{0}, {1}!", "Hello", "world"); // Hello, world!
    println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]);
    /*
        [
            1,
            2,
            3
        ]
     */

    // The format! macro is used to store the formatted string.
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    // print!() instead of println!()
    print!("Hello, world!"); // Without new line
    println!(); // New line

    print!("Hello, world!\n");
}