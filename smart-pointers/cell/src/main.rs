use std::cell::{Cell, RefCell};

fn main() {
    let imu = ImmutableStruct { id: 1 };
    //imu.id = 2; is not mutable
    println!("{:?}", imu);

    let mut muta = ImmutableStruct { id: 1 };
    muta.id = 2; //struct declared mutable
    println!("{:?}", muta);

    let intmu = InteriorMutable { id: Cell::new(1) };
    intmu.id.set(2); //mutate through Cell
    println!("{:?}", intmu);

    let intmuref = InteriorMutableRef {
        id: RefCell::new(1),
    };
    *intmuref.id.borrow_mut() = 2; //mutate through RefCell ref
}

#[derive(Debug)]
struct ImmutableStruct {
    pub id: i64,
}

#[derive(Debug)]
struct InteriorMutable {
    pub id: Cell<i64>,
}

#[derive(Debug)]
struct InteriorMutableRef {
    pub id: RefCell<i64>,
}
