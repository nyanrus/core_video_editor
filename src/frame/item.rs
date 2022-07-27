use std::{cell::RefCell, sync::{Arc, Mutex}, rc::Rc, borrow::Borrow};

use ulid::Ulid;

use super::frame::*;

pub enum ItemChild {
    FI(Box<dyn FrameInterface+Send+Sync>),
    Item(Item),
}

pub struct Item {
    pub id:Ulid,
    pub vec_child:Vec<Arc<ItemChild>>,
    pub parent:Option<Ulid>,
}

impl Default for Item {
    fn default() -> Self{
        Self { id: ulid::Ulid::new() , vec_child: Vec::new(), parent: None }
    }
}

impl FrameInterface for Item {
    fn process(&self,f:Option<&Frame>) -> Result<Option<Frame>, String> {
        let mut ff = (*f.unwrap()).clone();
        if self.vec_child.len() == 0 {
            return Err("No Child".to_string())
        }
        for i in &self.vec_child {
            match &*i.borrow() {
                ItemChild::FI(fi) => {
                    ff = fi.process(Some(&ff)).unwrap().unwrap();
                },
                ItemChild::Item(item) => {
                    ff = item.process(Some(&ff)).unwrap().unwrap();
                },
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