struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {

    let square = Rectangle::square(5);
    println!("Area: {}", square.area());

    let rectangle = Rectangle {
        width: 50,
        height: 20,
    };
    println!("Area: {}", rectangle.area());
}