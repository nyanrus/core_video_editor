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

use super::{item::{Item,ItemChild}, frame::{FrameInterface, Frame}};

use ulid::Ulid;

use serde_json::*;

struct ItemManager {
    id:Ulid,
    map:HashMap<Ulid,Item>,
}

impl FrameInterface for ItemManager {
    fn get_settings(&self) -> String {
        let a = serde_json::json!("[]");
        self.map.iter().for_each(|(&i,v)|{
          let m = Map::from_iter(HashMap::from([(i.to_string(),serde_json::from_str::<serde_json::Value>(&v.get_settings()).unwrap())]));
          //m.insert(i.to_string(), serde_json::from_str(&v.get_settings()).unwrap());
          let _ = a.as_array().insert(&vec![serde_json::Value::Object(m)]);
        }
        );
        a.to_string()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&self,f:&mut Frame,json:&str) -> bool {
        for i in &self.map {
          i.1.process(f, json);
        }

        true
    }
}

#[allow(dead_code)]
impl ItemManager {
    fn add(&mut self,item:Item) -> Ulid{
        let id = item.id;
        self.map.insert(item.id, item);
        id
    }

    fn del(&mut self,id:&Ulid) {
        self.map.remove(id);
    }

    fn add_child(&mut self,parent:&mut Item,child:ItemChild) -> Ulid{
        let c_id = match &child {
            ItemChild::FI(fi) => fi.get_ulid(),
            ItemChild::Item(item) => item.id,
        };
        parent.map_child.insert(c_id, child);
        c_id
    }

    fn del_child(&mut self,parent:&mut Item,child_id:&Ulid) {
        parent.map_child.remove(child_id);
    }

    fn get(&self,id:&Ulid) -> Option<&Item>{
        self.map.get(id)
    }

    fn get_mut(&mut self,id:&Ulid) -> Option<&mut Item> {
        self.map.get_mut(id)
    }

    fn mov(&mut self,id:&Ulid,layer:usize,lr:(usize,usize)) -> bool{
        let collid = self.map.iter().any(|f|
          ((*f.1).layer == layer) && Self::is_collid(lr,(*f.1).lr)
        );

        if collid {
          false
        } else {
          self.map.get_mut(id).unwrap().layer = layer;
          self.map.get_mut(id).unwrap().lr = lr;
          true
        }
    }

    fn is_collid(lr1:(usize,usize),lr2:(usize,usize)) -> bool {
      !((lr1.1 < lr2.0) || (lr2.1 < lr1.0))
    }
}