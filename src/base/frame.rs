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

use std::rc::Rc;

use serde_json as json;

#[derive(Debug, Clone)]
pub struct Frame {
    pub w: usize, // almost u32 but usize is useful to make vec
    pub h: usize,
    pub vec_rgba: Vec<u8>,
} // RGBA

impl Frame {
    pub fn init(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            vec_rgba: Vec::<u8>::new(),
        }
    }
}

pub struct FrameSettings {
    pub frame_num: usize,
    pub w: usize,
    pub h: usize,
    pub child: Option<Vec<Rc<FrameSettings>>>,
    pub metadata: json::Value,
}
