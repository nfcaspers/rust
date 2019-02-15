const PI: f64 = 3.14;

fn main() {
    let result = calc_area(PI, 10.0);
    println!("Result is: {}", result);
}

//pure function
fn calc_area(pi: f64, radius: f64) -> f64 {
    radius * radius * pi
}
