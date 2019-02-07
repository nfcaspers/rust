use std::ops::{Add, Sub, Mul, Div};

fn main() {
    //Add
    assert_eq!(Point {x: 3, y: 3}, Point {x: 1, y: 0} + Point {x: 2, y: 3});

    //Sub
    assert_eq!(Point {x: -1, y: -3}, Point {x: 1, y: 0} - Point {x: 2, y: 3});

    //Mul
    assert_eq!(SomeInt {x: 25}, SomeInt {x: 5} * SomeInt {x: 5});

    //Div
    assert_eq!(SomeInt {x: 1}, SomeInt {x: 5} / SomeInt {x: 5});
}



#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(Debug, PartialEq)]
struct SomeInt {
    x: i32,
}

impl Mul for SomeInt {
    type Output = SomeInt;

    fn mul(self, other: SomeInt) -> SomeInt {
        SomeInt {x: self.x * other.x}
    }
}

impl Div for SomeInt {
    type Output = SomeInt;

    fn div(self, other: SomeInt) -> SomeInt {
        SomeInt {x: self.x / other.x }
    }
}
