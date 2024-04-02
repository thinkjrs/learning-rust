#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn make_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width1, height1))
    );

    let rect1 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area of the {rect1:#?} is {} square pixels.",
        area3(&rect1)
    );

    println!(
        "The area of the {rect1:#?} is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 40,
        height: 42,
    };

    let rect3 = Rectangle::make_square(15);

    println!(
        "Can {rect1:#?} hold {rect2:#?}?: {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can {rect2:#?} hold {rect1:#?}?: {}",
        rect2.can_hold(&rect1)
    );

    println!(
        "Can {rect2:#?} hold {rect3:#?}?: {}",
        rect2.can_hold(&rect3)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
