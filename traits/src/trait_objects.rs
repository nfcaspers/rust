use std::fmt::Debug;

pub fn trait_object() {
    print!("\nTrait Objects:\n");
    //This variable can hold any type implementing trait Foo
    //Because the size of the type being hold can not be determined
    //at compile time it has to be stored on the heap through the use of a box pointer
    let x: Box<dyn Foo> = Box::new(BasicObject {
        some_field: String::from("just some field"),
    });
    x.bar();
}

pub trait Foo: Debug {
    fn bar(&self);
}

#[derive(Debug)]
struct BasicObject {
    some_field: String,
}

impl Foo for BasicObject {
    fn bar(&self) {
        println!("This object is: {:?}", self)
    }
}
