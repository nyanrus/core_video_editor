use std::{collections::HashMap, sync::Arc};

use super::{item::{Item,ItemChild}, frame::FrameInterface};

use ulid::Ulid;

struct ItemManager {
    id:Ulid,
    map:HashMap<Ulid,Item>,
}

impl FrameInterface for ItemManager {
    fn get_settings(&self) -> String {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&self,f:Option<super::frame::Frame>,json:&str) -> Result<Option<super::frame::Frame>,String> {
        let mut ff : Option<super::frame::Frame> = f;
        for i in self.map.iter() {
          ff = (*i.1).process(ff, json)?;
        }

        return Ok(ff);
    }
}

impl ItemManager {
    fn add(&mut self,item:Item) {
        self.map.insert(item.id, item);
    }

    fn del(&mut self,id:&Ulid) {
        self.map.remove(&id);
    }

    fn add_child(&mut self,parent:&mut Item,child:ItemChild) -> Ulid{
        let c_id = match &child {
            ItemChild::FI(fi) => {
              fi.get_ulid()
            },
            ItemChild::Item(item) => {
              item.id
            },
        };
        parent.map_child.insert(c_id, child);
        return c_id
    }

    fn del_child(&mut self,parent:&mut Item,child_id:&Ulid) {
        parent.map_child.remove(child_id);
    }
}