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

use crate::base::item::ItemChild;

//FrameInterfaceInfo
//But using Itemchild, you can use Item too
pub struct FiInfo {
    pub name: String,
    pub authors: Box<[String]>,
    pub url: Box<[String]>,
    pub version: String,
    pub tag: Box<[String]>,
    pub fi: ItemChild,
}

pub struct FiManager {}

impl FiManager {
    pub fn register(&mut self, info: FiInfo) {}
}