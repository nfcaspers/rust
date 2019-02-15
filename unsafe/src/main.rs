fn main() {
    let x: i64 = 7;
    unsafe {
        let ptr = &x as *const i64;
        println!("{:?}", *ptr);
    }
}
