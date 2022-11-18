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

use super::{
    frame::{self, Frame, FrameSettings},
    item,
};
use crate::base::interface::ProcessInterface;
use serde_json as json;
use std::collections::HashMap;
use ulid::Ulid;
pub struct Layer<TData, TSettings> {
    pub id: Ulid,
    pub name: String,
    pub map_item: HashMap<Ulid, LayerItem<TData, TSettings>>,
}

pub struct LayerItem<TData, TSettings> {
    pub item: item::Item<TData, TSettings>,
    pub lr: (usize, usize),
}

#[allow(dead_code)]
impl Layer<Frame, FrameSettings> {
    fn get_item_by_num(&self, frame_num: usize) -> Option<Ulid> {
        return self
            .map_item
            .iter()
            .find(|(_, item)| item.lr.0 <= frame_num && item.lr.1 > frame_num)
            .map(|s| *s.0);
    }

    fn add(&mut self, layer_item: LayerItem<Frame, FrameSettings>) -> Ulid {
        let id = layer_item.item.get_ulid();
        self.map_item.insert(id, layer_item);
        id
    }

    fn del(&mut self, id: &Ulid) {
        self.map_item.remove(id);
    }

    fn get(&self, id: &Ulid) -> Option<&LayerItem<Frame, FrameSettings>> {
        self.map_item.get(id)
    }

    fn get_mut(&mut self, id: &Ulid) -> Option<&mut LayerItem<Frame, FrameSettings>> {
        self.map_item.get_mut(id)
    }

    fn mov(&mut self, id: &Ulid, lr: (usize, usize)) -> bool {
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

impl ProcessInterface<Frame, FrameSettings> for Layer<Frame, FrameSettings> {
    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(
        &mut self,
        f: &mut Frame,
        settings: &frame::FrameSettings,
        json: json::Value,
    ) -> bool {
        let id = self.get_item_by_num(settings.frame_num);
        match id {
            Some(s) => {
                let a: &mut LayerItem<Frame, FrameSettings> = self.map_item.get_mut(&s).unwrap();
                a.item.process(f, settings, json)
            }
            None => false,
        }
    }

    fn get_json_template(&self) -> json::Value {
        todo!()
    }
}
