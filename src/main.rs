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

// use opencv::{
// 	core::{self},
// };
// opencv::opencv_branch_4! {
// 	use opencv::core::AccessFlag::ACCESS_READ;
// }
// opencv::not_opencv_branch_4! {
// 	use opencv::core::ACCESS_READ;
// }

use std::time::Instant;

//mod frame;
//use frame::cvvideo;

use core_video_editor::{
    backend::cvvideo::*,
    base::frame::{Frame, Settings},
    io::{input::InputInterface, output::OutputInterface},
};
use rayon::prelude::*;
use serde_json as json;

fn main() {
    test_calc();
}

fn test_calc() {
    println!("");
    let mut a = Vec::new();
    for i in 0..2_073_600_000 {
        a.push(i as f32 / 255.);
        //print!("{}", i as f32 / 100_555.);
    }
    println!("{:?}", a);
    println!("end");
}

fn test_opencv() {
    let a = IOpenCV {};
    let mut f = Frame::init(1920, 1080);

    let b = a.in_open_file("1.mp4").unwrap();
    let now = Instant::now();
    let mut i = 0;

    let v = a.out_open_file("2.mp4").unwrap();
    loop {
        let c = b.process(
            &mut f,
            &Settings {
                frame_num: i,
                w: 1920,
                h: 1080,
            },
            &json::json!({}),
        );
        println!("{} {}", i, c);
        if !c {
            break;
        }
        v.process(
            &mut f,
            &Settings {
                frame_num: 0,
                w: 1920,
                h: 1080,
            },
            &json::json!({}),
        );
        i += 1;
    }
}
