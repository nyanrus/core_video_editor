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

use crate::{
    base::frame::Frame,
    io::{filter::FilterInterface, input::InputInterface, output::OutputInterface},
};

pub enum FiChild<T> {
    Input(Box<dyn InputInterface<T>>),
    Output(Box<dyn OutputInterface<T>>),
    Filter(Box<dyn FilterInterface<T>>),
}

//FrameInterfaceInfo
//But using Itemchild, you can use Item too
pub struct FiInfo<T> {
    pub name: String,
    pub authors: Box<[String]>,
    pub url: Box<[String]>,
    pub version: String,
    pub tag: Box<[String]>,
    pub fi: FiChild<T>,
}

pub struct FiManager<T> {
    pub vec_info: Vec<FiInfo<T>>,
}

impl FiManager<Frame> {
    pub fn register(&mut self, info: FiInfo<Frame>) {
        self.vec_info.push(info);
    }
}
