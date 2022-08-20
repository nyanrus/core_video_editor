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

use opencv::{
    prelude::MatTraitConstManual,
    videoio::{VideoWriter, VideoWriterTrait, CAP_FFMPEG},
};
use rgb::{ComponentBytes, FromSlice};
use std::{sync::Mutex, time::Instant};

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
    let a = IOpenCV {};
    let mut f = Frame::init(1920, 1080);

    let b = a.in_open_file("1.mp4").unwrap();
    let now = Instant::now();
    let mut i = 0;

    b.process(
        &mut f,
        &Settings {
            frame_num: 0,
            w: 1920,
            h: 1080,
        },
        &json::json!({}),
    );

    let v = a.out_open_file("2.mp4").unwrap();
    loop {
        let b = b.process(
            &mut f,
            &Settings {
                frame_num: i,
                w: 1920,
                h: 1080,
            },
            &json::json!({}),
        );
        // println!("{} {}", i, b);
        if !b {
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
    //drop(v);

    // let ddd = get_video_capture("1.mp4").unwrap();

    // // let mut vw = opencv::videoio::VideoWriter::default().unwrap();
    // // println!(
    // //     "{}",
    // //     vw.open(
    // //         "2.mp4",
    // //         i32::from_ne_bytes(*b"mp4v"),
    // //         30.,
    // //         opencv::core::Size {
    // //             width: 1920,
    // //             height: 1080,
    // //         },
    // //         true,
    // //     )
    // //     .unwrap()
    // // );

    // let o = a.out_open_file("2.mp4").unwrap();

    // o.process(
    //     &mut Frame {
    //         w: 1920,
    //         h: 1080,
    //         vec_rgba: get_video_frame(&Mutex::new(ddd), 0.)
    //             .data_bytes()
    //             .unwrap()
    //             .par_iter()
    //             .map(|x| *x as f32 / 255.)
    //             .chunks(3)
    //             .map(|x| [x[2], x[1], x[0], 1.])
    //             .flatten_iter()
    //             .collect::<Vec<f32>>()
    //             .as_rgba()
    //             .to_vec(),
    //     },
    //     &Settings {
    //         frame_num: 0,
    //         w: 1920,
    //         h: 1080,
    //     },
    //     &json::json!({}),
    // );
    // drop(o);

    //vw.open(filename, fourcc, fps, frame_size, is_color)

    //vw.write().unwrap();

    //vw.release().unwrap();
}
