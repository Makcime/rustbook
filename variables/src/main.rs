use std::io;


const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main() {

    // mutable varialbles
    let mut x = 5;
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");


    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in th einner scope is: {y}")
    }

    println!("The value of y is : {y}");

    let a = 2.0; // f64
    let b: f32 = 3.0; // f32

    // Numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Characters type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Use arrays rather than vectors, when you know the size you need
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

    // Accessing Array Elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first element of a is {first} and the second is {second}");


    // invalid element Accessing
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");


    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}")
}
