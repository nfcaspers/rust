#![allow(dead_code)]

#[derive(Clone, Copy)]
struct ErrInfo;

union ErrUnion {
    recover: ErrInfo, 
    fail: u8,
}

fn main() {
    //cast ref to pointer and deref it
    let x: i64 = 7;
    unsafe {
        let ptr = &x as *const i64;
        println!("Derefed pointer: {:?}", *ptr);
    }

    //use union
    let e = ErrUnion{ fail: 100 };
    unsafe {
        let err_val = e.fail;
        println!("Value of Error is: {}", err_val)
    }
}
