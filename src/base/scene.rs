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

use super::frame::FrameSettings;
use super::interface::ProcessInterface;
use super::{frame::Frame, layer::Layer};
use serde_json as json;
use ulid::Ulid;

struct Scene<TData, TSettings> {
    id: Ulid,
    map: HashMap<Ulid, Layer<TData, TSettings>>,
}

impl ProcessInterface<Frame, FrameSettings> for Scene<Frame, FrameSettings> {
    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(
        &mut self,
        f: &mut Frame,
        settings: &super::frame::FrameSettings,
        json: json::Value,
    ) -> bool {
        for i in &mut self.map {
            i.1.process(f, settings, json.clone());
        }
        true
    }

    fn get_json_template(&self) -> json::Value {
        let a = json::json!("[]");
        self.map.iter().for_each(|(&i, v)| {
            let m = json::Map::from_iter(HashMap::from([(i.to_string(), v.get_json_template())]));
            //m.insert(i.to_string(), json::from_str(&v.get_settings()).unwrap());
            let _ = a.as_array().insert(&vec![json::Value::Object(m)]);
        });
        a
    }
}
