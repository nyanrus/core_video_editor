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

use super::{
    frame::{Frame, FrameInterface},
    layer::Layer,
};

use serde_json as json;
use ulid::Ulid;

struct Scene {
    id: Ulid,
    map: HashMap<Ulid, Layer>,
}

impl FrameInterface for Scene {
    fn get_settings(&self) -> json::Value {
        let a = json::json!("[]");
        self.map.iter().for_each(|(&i, v)| {
            let m = json::Map::from_iter(HashMap::from([(i.to_string(), v.get_settings())]));
            //m.insert(i.to_string(), json::from_str(&v.get_settings()).unwrap());
            let _ = a.as_array().insert(&vec![json::Value::Object(m)]);
        });
        a
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(
        &self,
        f: &mut Frame,
        settings: &super::frame::Settings,
        json: &json::Value,
    ) -> bool {
        for i in &self.map {
            i.1.process(f, settings, json);
        }
        true
    }
}
