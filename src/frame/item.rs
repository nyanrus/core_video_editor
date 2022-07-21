use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::frame::*;

struct Item {
    ulid:String,
    vec_child:Vec<Option<Rc<RefCell<Item>>>>,
    parent:Option<Weak<RefCell<Item>>>,
}