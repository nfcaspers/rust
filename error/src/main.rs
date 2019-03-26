#![allow(unused_variables)]

struct Foo {
    _field: String,
}

impl Foo {
    fn new_fail() -> Result<Foo, FooError> {
        Err(FooError)
    }
    fn new_succeed() -> Result<Foo, FooError> {
        Ok(Foo{_field: String::from("Ok!")})
    }
}

#[derive(Debug)]
struct FooError;


fn main() -> Result<(), FooError> {
    let f = Foo::new_succeed(); // is wrapped in result 
    //unwrap result with match
    match f {
        Ok(foo) => foo, 
        Err(foo_err) => return Err(foo_err), //very verbose
    };
    //unwrap Result if Ok or panic if Err
    let x = Foo::new_succeed().unwrap(); 

    // 'syntax sugar' for unwraping or returning early
    let y = Foo::new_succeed()?; 
    Ok(())
}