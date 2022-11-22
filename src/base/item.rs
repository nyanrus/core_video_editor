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

use super::{frame::*, interface::ProcessInterface};

pub enum ItemChild<TData, TSettings> {
    FI(Box<dyn ProcessInterface<TData, TSettings> + Send + Sync>),
    Item(Box<Item<TData, TSettings>>),
}

impl ProcessInterface<Frame, FrameSettings> for ItemChild<Frame, FrameSettings> {
    fn get_json_template(&self) -> json::Value {
        match self {
            ItemChild::FI(fi) => fi.get_json_template(),
            ItemChild::Item(i) => i.get_json_template(),
        }
    }

    fn get_ulid(&self) -> Ulid {
        match self {
            ItemChild::FI(fi) => fi.get_ulid(),
            ItemChild::Item(i) => i.get_ulid(),
        }
    }

    fn process(&mut self, f: &mut Box<Frame>, settings: &FrameSettings, json: json::Value) -> bool {
        match self {
            ItemChild::FI(fi) => fi.process(f, settings, json),
            ItemChild::Item(i) => i.process(f, settings, json),
        }
    }
}

pub struct Item<TData, TSettings> {
    pub id: Ulid,
    pub map_child: HashMap<Ulid, ItemChild<TData, TSettings>>,
}

impl Default for Item<Frame, FrameSettings> {
    fn default() -> Self {
        Self {
            id: Ulid::new(),
            map_child: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl Item<Frame, FrameSettings> {
    fn add_child(
        &mut self,
        parent: &mut Item<Frame, FrameSettings>,
        child: ItemChild<Frame, FrameSettings>,
    ) -> Ulid {
        let c_id = child.get_ulid();
        parent.map_child.insert(c_id, child);
        c_id
    }

    fn del_child(&mut self, parent: &mut Item<Frame, FrameSettings>, child_id: &Ulid) {
        parent.map_child.remove(child_id);
    }
}

impl ProcessInterface<Frame, FrameSettings> for Item<Frame, FrameSettings> {
    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&mut self, f: &mut Box<Frame>, settings: &FrameSettings, json: json::Value) -> bool {
        if self.map_child.is_empty() {
            return false;
        }
        self.map_child.iter_mut().for_each(|(_id, child)| {
            child.process(f, settings, json.clone());
        });
        true
    }

    fn get_json_template(&self) -> json::Value {
        let a = json::json!("[]");
        self.map_child.iter().for_each(|(&_i, v)| match v {
            ItemChild::FI(_f) => todo!(),
            ItemChild::Item(_item) => {
                todo!()
            }
        });
        a
    }
}
