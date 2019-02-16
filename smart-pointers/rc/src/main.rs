use std::rc::{Rc, Weak};

fn main() {
    let owner = Owner::new();
    let weakref = WeakRef::new(&owner);

    drop(owner);
    //if assert is true the Value has been dropped
    assert!(weakref._ref.upgrade().is_none());
}

#[derive(Debug)]
struct Owner {
    pub owned: Rc<i32>,
}

impl Owner {
    fn new() -> Owner {
        Owner { owned: Rc::new(10) }
    }
}

#[derive(Debug)]
struct WeakRef {
    pub _ref: Weak<i32>,
}

impl WeakRef {
    fn new(value_owner: &Owner) -> WeakRef {
        WeakRef {
            _ref: Rc::downgrade(&value_owner.owned),
        }
    }
}
