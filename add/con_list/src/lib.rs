use std::rc::Rc;
use std::cell::RefCell;
// pub enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}