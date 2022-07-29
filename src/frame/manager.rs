use std::{collections::HashMap, sync::Arc};

use super::item::{Item,ItemChild};

use ulid::Ulid;

struct ItemManager {
    map:HashMap<Ulid,Item>
}

impl ItemManager {
    fn add(&mut self,item:Item) {
        self.map.insert(item.id, item);
    }

    fn del(&mut self,id:&Ulid) {
        self.map.remove(&id);
    }

    fn set_child(&mut self,parent:&mut Item,child:ItemChild) -> Ulid{
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