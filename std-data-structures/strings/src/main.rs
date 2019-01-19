#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    let s1 = String::new(); //create a new String allocated on the heap

    let data = "2 plus 2 is 4 minus 1 is 3 -> Quick Maths!"; //create a string constant
    let s2 = data.to_string(); //convert the string constant to a 'normal' String
    let s3 = "This is false.".to_string(); //create a String from a &str
    let mut s4 = String::from("This is true."); //create a mutable String from a &str
    s4.push_str("False"); //append &str to String

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; //combine two Strings
    println!("= {}", s7);
    println!("s6 is {}",s6 );

    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s11 = format!("{}-{}-{}", s8, s9, s10); //format Strings with a macro

    for c in s11.chars() { //print chars of a string
        println!("{}", c);
    }
    for c in s11.bytes() { //print bytes of a string
        println!("{}", c);
    }
}
