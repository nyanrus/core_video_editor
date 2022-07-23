use std::{cell::RefCell, rc::{Rc, Weak}, fmt::Error};

use super::frame::*;

enum ItemChild {
    FI(Box<dyn FrameInterface>),
    Item(Item),
}

struct Item {
    ulid:String,
    vec_child:Vec<Option<Rc<RefCell<ItemChild>>>>,
    parent:Option<Weak<RefCell<Item>>>,
}


impl FrameInterface for Item {
    fn process(&self,f:Option<&Frame>) -> Result<Option<Frame>, String> {
        let mut ff = (*f.unwrap()).clone();
        for i in &self.vec_child {
            match i {
                Some(s) => {
                    match &*s.borrow() {
                        ItemChild::FI(fi) => {
                            ff = fi.process(Some(&ff)).unwrap().unwrap();
                        },
                        ItemChild::Item(item) => {
                            ff = item.process(Some(&ff)).unwrap().unwrap();
                        },
                    }
                },
                None => return Err("No Child".to_string()),
            }
        };
        return Ok(Some(ff));
    }

    fn get_settings(&self) -> String {
        todo!()
    }

    fn set_settings(&self,json:String) -> Result<(),String> {
        todo!()
    }
}