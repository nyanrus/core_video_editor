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

    fn process(&self,f:&mut Frame,json:&str) -> bool {
        for i in &self.map {
          i.1.process(f, json);
        }

        return true;
    }
}

impl ItemManager {
    fn add(&mut self,item:Item) -> Ulid{
        let id = item.id.clone();
        self.map.insert(item.id, item);
        return id
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
        return c_id
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
        let mut collid = false;
        let collid = self.map.iter().any(|f|
          ((*f.1).layer == layer) && Self::is_collid(lr,(*f.1).lr)
        );

        if collid {
          return false;
        } else {
          self.map.get_mut(id).unwrap().layer = layer;
          self.map.get_mut(id).unwrap().lr = lr;
          return true;
        }
    }

    fn is_collid(lr1:(usize,usize),lr2:(usize,usize)) -> bool {
      !((lr1.1 < lr2.0) || (lr2.1 < lr1.0))
    }
}