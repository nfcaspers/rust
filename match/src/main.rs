#![allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match &self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let akcoin = Coin::Quarter(UsState::Alaska).value_in_cents();
    println!("This Quarter has the value of {:?} cents.", akcoin)
}
