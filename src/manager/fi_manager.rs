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

use crate::io::{filter::FilterInterface, input::InputInterface, output::OutputInterface};

pub enum FiChild<TData, TSettings> {
    Input(Box<dyn InputInterface<TData, TSettings>>),
    Output(Box<dyn OutputInterface<TData, TSettings>>),
    Filter(Box<dyn FilterInterface<TData, TSettings>>),
}

//ProcessInterfaceInfo
pub struct FiInfo<TData, TSettings> {
    pub name: String,
    pub authors: Box<[String]>,
    pub url: Box<[String]>,
    pub version: String,
    pub tag: Box<[String]>,
    pub fi: FiChild<TData, TSettings>,
}

pub struct FiManager<TData, TSettings> {
    pub vec_info: Vec<FiInfo<TData, TSettings>>,
}

pub trait FiManage<TData, TSettings> {
    fn register(&mut self, info: FiInfo<TData, TSettings>);
}

impl<TData: Sized, TSettings: Sized> FiManage<TData, TSettings> for FiManager<TData, TSettings> {
    fn register(&mut self, info: FiInfo<TData, TSettings>) {
        self.vec_info.push(info);
    }
}
