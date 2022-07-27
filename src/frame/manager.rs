use std::{collections::HashMap, sync::Mutex, rc::Rc};

use super::item::Item;
use once_cell::sync::Lazy;
use ulid::Ulid;

static MANAGER: Lazy<Mutex<Manager>> = Lazy::new(|| {
    return Mutex::new(Manager{map:HashMap::new()});
});

struct Manager {
    map:HashMap<Ulid,Item>
}

impl Manager {
    fn register_item(item:Item) {
        MANAGER.lock().unwrap().map.insert(item.id, item);
    }

    fn del_item(id:&Ulid) {
        MANAGER.lock().unwrap().map.remove(&id);
    }
}