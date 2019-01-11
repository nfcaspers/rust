#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u64,
    length: u64,
}

impl Rectangle {
    fn square(size: u64) -> Rectangle {
        Rectangle {width:size,length:size}
    }
     fn area(&self) -> u64 {
         self.length * self.width
     }
     fn can_hold(&self, other: &Rectangle) -> bool {
         self.length > other.length && self.width > other.width
     }
}

fn main() {
    let rect1 = Rectangle { width: 30, length: 50 };
    let rect2 = Rectangle { width: 10, length: 40 };
    let rect3 = Rectangle { width: 60, length: 45 };

    let sq1 = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
