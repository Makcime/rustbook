struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    let rect1 = Rectangle {
        w: 30,
        h: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.w * rectangle.h
}
