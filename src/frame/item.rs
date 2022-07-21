use std::{cell::RefCell, rc::{Rc, Weak}, fmt::Error};

use super::frame::*;

struct Item {
    ulid:String,
    vec_child:Vec<Option<Rc<RefCell<Item>>>>,
    parent:Option<Weak<RefCell<Item>>>,
    filter_process:Option<Box<dyn FrameInterface>>,
}

trait ProcessFrame {
    fn process(&self,f:Option<&Frame>) -> Result<Option<&Frame>, Error>;
}

impl ProcessFrame for Item {
    fn process(&self,f:Option<&Frame>) -> Result<Option<&Frame>, Error> {
        match &(*self).filter_process {
            Some(o) => {
                o.process_frame(f)
            },
            None => Err(Error),
        }
    }
}