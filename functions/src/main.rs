fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // playing with expressions and statements

    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons.
              // If you add a semicolon to the end of an expression,
              // you turn it into a statement, and it will then not return a value.
              // Keep this in mind as you explore function return values and expressions next.
    };

    println!("The value of y is : {y}");

    // Functions that return Values

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // ; ==> implicitly returns `()` as its body has no tail or `return` expression
}

