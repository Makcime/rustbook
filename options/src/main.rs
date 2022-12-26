fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number : Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This won't compile Option<T> + T is not permited
    let sum = x + y;
}
