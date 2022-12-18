#[derive(Debug)]

struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn square(size: u32) -> self {
        Self {
            w: size,
            h: size,
        }
    }

    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn width(&self) -> bool {
        self.w > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        w: dbg!(15 * scale),
        h: 50,
    };

    let rect2 = Rectangle {
        w: 10,
        h: 40,
    };

    let rect3 = Rectangle {
        w: 60,
        h: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area()
        );

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero wigth; it is {}", rect1.w);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.w * rectangle.h
}
