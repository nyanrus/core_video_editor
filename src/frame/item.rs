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

use super::frame::*;

pub enum ItemChild {
    FI(Box<dyn FrameInterface+Send+Sync>),
    Item(Box<Item>),
}

pub struct Item {
    pub id:Ulid,
    pub map_child:HashMap<Ulid,ItemChild>,
    pub layer:usize,
    pub lr : (usize,usize),
}

impl Default for Item {
    fn default() -> Self{
        Self { id: ulid::Ulid::new() , map_child: HashMap::new(), layer: 0, lr: (0,0) }
    }
}

impl FrameInterface for Item {
    fn process(&self,f:&mut Frame,json:&str) -> bool {
        if self.map_child.is_empty() {
            return false;
        }
        self.map_child.iter().for_each(|(_id,child)| {
            match child {
                ItemChild::FI(fi) => {
                    fi.process(f,json);
                },
                ItemChild::Item(item) => {
                    item.process(f,json);
                },
            }
        });
        true
    }

    fn get_settings(&self) -> String {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }
}