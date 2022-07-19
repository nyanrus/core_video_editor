use std::pin::Pin;

use crate::frame::*;

struct Item {
    ulid:String,
    vec_child:Vec<Pin<Box<Item>>>,
}