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

use ulid::Ulid;

use super::{
    frame::{self, FrameInterface},
    item,
};

pub struct Layer {
    pub id: Ulid,
    pub name: String,
    pub map_item: HashMap<Ulid, LayerItem>,
}

pub struct LayerItem {
    pub item: item::Item,
    pub lr: (usize, usize),
}

#[allow(dead_code)]
impl Layer {
    fn get_item_by_num(&self, frame_num: usize) -> Option<&Ulid> {
        return match self
            .map_item
            .iter()
            .find(|(id, item)| item.lr.0 <= frame_num && item.lr.1 > frame_num)
        {
            Some(s) => Some(s.0),
            None => None,
        };
    }

    fn add(&mut self, layer_item: LayerItem) -> Ulid {
        let id = layer_item.item.get_ulid();
        self.map_item.insert(id, layer_item);
        id
    }

    fn del(&mut self, id: &Ulid) {
        self.map_item.remove(id);
    }

    fn get(&self, id: &Ulid) -> Option<&LayerItem> {
        self.map_item.get(id)
    }

    fn get_mut(&mut self, id: &Ulid) -> Option<&mut LayerItem> {
        self.map_item.get_mut(id)
    }

    fn mov(&mut self, id: &Ulid, layer: usize, lr: (usize, usize)) -> bool {
        let non_collid = !self.map_item.iter().any(|f| Self::is_collid(lr, f.1.lr));

        if non_collid {
            let item = self.map_item.get_mut(id).unwrap();
            item.lr = lr;
        }
        non_collid
    }

    fn is_collid(lr1: (usize, usize), lr2: (usize, usize)) -> bool {
        !((lr1.1 < lr2.0) || (lr2.1 < lr1.0))
    }
}

impl FrameInterface for Layer {
    fn get_settings(&self) -> serde_json::Value {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(
        &self,
        f: &mut frame::Frame,
        settings: &frame::Settings,
        json: &serde_json::Value,
    ) -> bool {
        match self.get_item_by_num(settings.frame_num) {
            Some(s) => {
                let a: &LayerItem = &self.map_item[s];
                a.item.process(f, settings, json)
            }
            None => false,
        }
    }
}
