use add_one;
use con_list::List::{Cons, Nil};


use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct MyInt {
    value: Rc<RefCell<i32>>
}
fn main() {
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    // println!("a before  = {:?}", a);

    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    let val = Rc::new(RefCell::new(5));
    let mut a =  Rc::new(MyInt { value: Rc::clone(&val) });
    let b = Rc::new(MyInt { value: Rc::clone(&val) });

    *val.borrow_mut() += 10;
    

    println!("val after = {:?}", val);
    print!("a after = {:?}", a);
    print!("b after = {:?}", b);
}

// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

// Rc<T> enables multiple owners of the same data;
// Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time;
//Rc<T> allows only immutable borrows checked at compile time;
//RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.