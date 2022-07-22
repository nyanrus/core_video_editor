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

trait ProcessFrame {
    fn process(&self,f:Option<&Frame>) -> Result<Option<&Frame>, Error>;
}

impl ProcessFrame for Item {
    fn process(&self,f:Option<&Frame>) -> Result<Option<&Frame>, Error> {
        let ff = f.unwrap();
        for i in self.vec_child {
            match i {
                Some(s) => {
                    match *s.borrow() {
                        ItemChild::FI(fi) => {
                            ff = fi.process_frame(Some(ff)).unwrap().unwrap();
                        },
                        ItemChild::Item(item) => {
                            ff = item.process(Some(ff)).unwrap().unwrap();
                        },
                    }
                },
                None => todo!(),
            }
        };
        return todo!();
    }
}