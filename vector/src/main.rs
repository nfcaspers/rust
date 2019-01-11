#![allow(dead_code)]
#![allow(unused_variables)]

enum FlexVector {
    Int(i32),
    Float(f64),
    Str(String),
}

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    let mut v3  = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);
    let third: Option<&i32> = v3.get(2);
    println!("Third is {:?}", third);
}
