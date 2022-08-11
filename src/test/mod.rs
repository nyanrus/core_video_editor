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

use std::sync::Mutex;

use serde_json as json;
use ulid::Ulid;

use crate::backend::cvvideo::{get_video_capture, CvFrameIn};

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use super::frame::frame::*;

pub fn a() {
    let mut vec = Vec::<Box<dyn FrameInterface>>::new();
    let a = CvFrameIn {
        vc: Mutex::new(get_video_capture("test.mp4").unwrap()),
        id: Ulid::new(),
    };
    vec.push(Box::new(a) as Box<dyn FrameInterface>);
    let mut f = Frame::init(1920, 1080);
    for i in vec {
        let _a = i.process(&mut f, &json::Value::Null);
        //println!("{:?}",a.unwrap());
    }
}