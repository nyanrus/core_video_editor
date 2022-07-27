use std::{collections::HashMap};

use super::item::Item;

use ulid::Ulid;

struct Manager {
    map:HashMap<Ulid,Item>
}

impl Manager {
    fn register_item(&mut self,item:Item) {
        self.map.insert(item.id, item);
    }

    fn del_item(&mut self,id:&Ulid) {
        self.map.remove(&id);
    }
}