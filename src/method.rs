#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn fs(&self) {
        println!("fs: {}", self.area());
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::square(30);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.fs();
    rect2.fs();
}
