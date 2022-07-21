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

use std::fmt::Error;


#[derive(Debug)]
pub struct Frame{
    pub w:u32,
    pub h:u32,
    pub pix_vec:Vec<u8>
} // RGBA

pub struct Settings{}

pub trait FrameInterface {
    fn get_settings(&self) -> String; //JSON
    fn set_settings(&self,json:String) -> Result<(),String>;
    fn process_frame(&self,f:Option<&Frame>) -> Result<Option<&Frame>,Error>;
}

// fn a() {
//     let a = frame{};
//     a.get_frame(settings{});
//     frame::get_frame(settings);
// }