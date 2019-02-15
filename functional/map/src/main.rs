fn main() {
    let numbers: Vec<i64> = vec![100, 200, 430, 25];
    let incremented: Vec<i64> = numbers.iter().map(|x| x + 1).collect();
    println!("{:?}", incremented);
}
