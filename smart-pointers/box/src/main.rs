#![allow(unused_variables)]

fn main() {
    let heap_alloc = Box::new(1337);
    let obj = Box::new(SomeStruct);
    take_trait_obj(obj);
}

fn take_trait_obj(x: Box<dyn Foo>) {
    x.bar();
}

trait Foo {
    fn bar(&self);
}

struct SomeStruct;

impl Foo for SomeStruct {
    fn bar(&self) {
        print!("Working")
    }
}
