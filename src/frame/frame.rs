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

use serde_json as json;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Frame {
    pub w: u32,
    pub h: u32,
    pub vec_rgba: Vec<[u8; 4]>,
} // RGBA

impl Frame {
    pub fn init(w: u32, h: u32) -> Self {
        Self {
            w,
            h,
            vec_rgba: Vec::<[u8; 4]>::with_capacity((w * h).try_into().unwrap()),
        }
    }
}

pub struct Settings {}

pub trait FrameInterface {
    fn get_settings(&self) -> json::Value; //JSON template
    fn get_ulid(&self) -> Ulid;
    fn process(&self, f: &mut Frame, json: &json::Value) -> bool;
}
