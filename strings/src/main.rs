fn main() {
    let mut s1 = String::new();
    let data = "2 plus 2 is 4 minus 1 is 3 -> Quick Maths!";
    let s2 = data.to_string();
    let s3 = "This is false.".to_string();
    let mut s4 = String::from("This is true.");
    s4.push_str("False");
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;
    println!("= {}", s7);
    println!("s6 is {}",s6 );
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10);
    for c in s11.chars() {
        println!("{}", c);
    }
    for c in s11.bytes() {
        println!("{}", c);
    }
}
