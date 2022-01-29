const PI: f64 = 3.14159265359;

fn main() {
    println!("The value of Pi is {}", PI);

    let a; // Declaration
    println!("{}", a);

    a = 5;
    println!("{}", a);

    let b: i8; // Declaration with data type
    println!("{}", b);

    b = 5;
    println!("{}", b);

    let t = true; // Data type is implicit
    println!("{}", t);

    let f: bool = false;
    println!("{}", f);

    let(x, y) = (1, 2); // x = 1 and y = 2
    println!("{}, {}", x, y);

    let mut z = 5; // Mutable variable
    println!("{}", z);
    z = 6;
    println!("{}", z);

    let z = {
        let x = 1;
        let y = 2;

        x + y
    }; // z = 3
    println!("{}", z);


    const M: i32 = 5;

    static N: i32 = 5;


    let g: f64 = -20.48; // float
    let h: i64 = g.floor() as i64; // int (rounded down)
    println!("{}", g); // -21

    let i: &str = "hello"; //&str (Unicode string slices)
    let i: String = i.to_uppercase(); // String
    println!("{}", i); // HELLO

}
