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

use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;
use serde_json as json;
use ulid::Ulid;

use super::{frame::*, interface::ProcessInterface};

pub enum ItemChild<TData, TSettings> {
    FI(Arc<dyn ProcessInterface<TData, TSettings> + Send + Sync>),
    Item(Arc<Item<TData, TSettings>>),
}

impl ProcessInterface<Frame, FrameSettings> for ItemChild<Frame, FrameSettings> {
    fn get_json_template(&self) -> anyhow::Result<json::Value> {
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

    fn process(
        &self,
        f: &mut Frame,
        settings: &FrameSettings,
        json: json::Value,
    ) -> anyhow::Result<bool> {
        match self {
            ItemChild::FI(fi) => fi.process(f, settings, json),
            ItemChild::Item(i) => i.process(f, settings, json),
        }
    }
}

pub struct Item<TData, TSettings> {
    pub id: Ulid,
    pub map_child: Arc<Mutex<HashMap<Ulid, ItemChild<TData, TSettings>>>>,
}

impl Default for Item<Frame, FrameSettings> {
    fn default() -> Self {
        Self {
            id: Ulid::new(),
            map_child: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[allow(dead_code)]
impl Item<Frame, FrameSettings> {
    fn add_child(&mut self, child: ItemChild<Frame, FrameSettings>) -> anyhow::Result<Ulid> {
        let c_id = child.get_ulid();

        match self.map_child.try_lock() {
            Some(mut o) => o.insert(c_id, child),
            None => return Err(anyhow::anyhow!("add_child lock_err")),
        };
        Ok(c_id)
    }

    fn del_child(&mut self, child_id: &Ulid) -> anyhow::Result<()> {
        match self.map_child.try_lock() {
            Some(mut o) => o.remove(child_id),
            None => return Err(anyhow::anyhow!("del_child lock err")),
        };
        Ok(())
    }
}

impl ProcessInterface<Frame, FrameSettings> for Item<Frame, FrameSettings> {
    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(
        &self,
        f: &mut Frame,
        settings: &FrameSettings,
        json: json::Value,
    ) -> anyhow::Result<bool> {
        if self
            .map_child
            .try_lock()
            .expect("item process lock err")
            .is_empty()
        {
            return Ok(false);
        }
        self.map_child
            .try_lock()
            .expect("item process lock err")
            .iter_mut()
            .for_each(|(_id, child)| {
                child.process(f, settings, json.clone());
            });
        Ok(true)
    }

    fn get_json_template(&self) -> anyhow::Result<json::Value> {
        let a = json::json!("[]");
        let bindings = self
            .map_child
            .try_lock()
            .expect("item get_json_template lock err");
        for (&_i, v) in bindings.iter() {
            match v {
                ItemChild::FI(_f) => todo!(),
                ItemChild::Item(_item) => {
                    todo!()
                }
            }
        }
        Ok(a)
    }
}
