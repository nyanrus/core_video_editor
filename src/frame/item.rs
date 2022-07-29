use std::{sync::Arc, borrow::Borrow, collections::HashMap};

use ulid::Ulid;

use super::frame::*;

pub enum ItemChild {
    FI(Box<dyn FrameInterface+Send+Sync>),
    Item(Box<Item>),
}

pub struct Item {
    pub id:Ulid,
    pub map_child:HashMap<Ulid,ItemChild>,
    pub layer:usize,
    pub lr : (usize,usize),
}

impl Default for Item {
    fn default() -> Self{
        Self { id: ulid::Ulid::new() , map_child: HashMap::new(), layer: 0, lr: (0,0) }
    }
}

impl FrameInterface for Item {
    fn process(&self,f:Option<Frame>,json:&str) -> Result<Option<Frame>, String> {
        let mut ff = f;
        if self.map_child.len() == 0 {
            return Err("No Child".to_string())
        }
        for (id,child) in &self.map_child {
            match child {
                ItemChild::FI(fi) => {
                    ff = fi.process(ff,json).unwrap();
                },
                ItemChild::Item(item) => {
                    ff = item.process(ff,json).unwrap();
                },
            }
        };
        return Ok(ff);
    }

    fn get_settings(&self) -> String {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        todo!()
    }
}