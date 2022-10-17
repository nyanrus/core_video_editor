// core_video_editor core of video editor, to develop easily
// Copyright (C) 2022 NyanRus

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use serde_json as json;
use ulid::Ulid;

use super::frame::*;

pub enum ItemChild<T> {
    FI(Box<dyn FrameInterface<T> + Send + Sync>),
    Item(Box<Item<T>>),
}

impl FrameInterface<Frame> for ItemChild<Frame> {
    fn get_settings(&self) -> json::Value {
        match self {
            ItemChild::FI(fi) => fi.get_settings(),
            ItemChild::Item(i) => i.get_settings(),
        }
    }

    fn get_ulid(&self) -> Ulid {
        match self {
            ItemChild::FI(fi) => fi.get_ulid(),
            ItemChild::Item(i) => i.get_ulid(),
        }
    }

    fn process(&self, f: &mut Frame, settings: &Settings, json: &json::Value) -> bool {
        match self {
            ItemChild::FI(fi) => fi.process(f, settings, json),
            ItemChild::Item(i) => i.process(f, settings, json),
        }
    }
}

pub struct Item<T> {
    pub id: Ulid,
    pub map_child: HashMap<Ulid, ItemChild<T>>,
}

impl Default for Item<Frame> {
    fn default() -> Self {
        Self {
            id: Ulid::new(),
            map_child: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl Item<Frame> {
    fn add_child(&mut self, parent: &mut Item<Frame>, child: ItemChild<Frame>) -> Ulid {
        let c_id = child.get_ulid();
        parent.map_child.insert(c_id, child);
        c_id
    }

    fn del_child(&mut self, parent: &mut Item<Frame>, child_id: &Ulid) {
        parent.map_child.remove(child_id);
    }
}

impl FrameInterface<Frame> for Item<Frame> {
    fn get_settings(&self) -> json::Value {
        let a = json::json!("[]");
        self.map_child.iter().for_each(|(&i, v)| match v {
            ItemChild::FI(_f) => todo!(),
            ItemChild::Item(item) => {
                let m = json::Map::from_iter(HashMap::from([(i.to_string(), item.get_settings())]));
                let _ = a.as_array().insert(&vec![json::Value::Object(m)]);
            }
        });
        a
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&self, f: &mut Frame, settings: &Settings, json: &json::Value) -> bool {
        if self.map_child.is_empty() {
            return false;
        }
        self.map_child.iter().for_each(|(_id, child)| {
            child.process(f, settings, json);
        });
        true
    }
}
