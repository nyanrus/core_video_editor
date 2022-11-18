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

// use duct::cmd;
use std::time::Instant;
//mod frame;
//use frame::cvvideo;

use anyhow::Result;
//use core_video_editor::backend::cvvideo::*

fn main() {
    test_opencv();
}

#[allow(dead_code)]
fn test_calc() {
    println!();
    let mut a = Vec::<f32>::with_capacity(1920 * 1080);
    let now = Instant::now();
    (0..2_073_600)
        .into_iter()
        .for_each(|i| a.push(i as f32 / 255.));
    //let b = i as f32 / 255.;

    //print!("{}", i as f32 / 100_555.);
    //println!("{:?}", a);
    println!("{}", now.elapsed().as_millis());
}

fn test_opencv() {
    //let a = IOpenCV {};
    // let mut f = Frame::init(1920, 1080);

    //let b = a.in_open_file("1.mp4").unwrap();
    let now = Instant::now();

    //let v = a.out_open_file("2.mp4").unwrap();
    ffmpeg_next::init().unwrap();
    let mut ctx = core_video_editor::backend::ffmpeg::FFInput::init("1.mp4").unwrap();
    let b_va = ctx.best_va;
    let mut aud_vec = Vec::new();
    core_video_editor::backend::ffmpeg::read::read_audio(&mut ctx, b_va.1, 0., &mut aud_vec)
        .unwrap();
    // loop {
    //     let mut c = true;
    //     if i == 7200 {
    //         c = false;
    //     }

    //     let mut f = core_video_editor::base::frame::Frame::init(1920, 1080);

    //     core_video_editor::backend::ffmpeg::read::read_video(&mut ctx, b_va.0, i, &mut f).unwrap();

    //     println!("{} {}", i, c);
    //     if !c {
    //         break;
    //     }
    //     // v.process(
    //     //     &mut f,
    //     //     &Settings {
    //     //         frame_num: i,
    //     //         w: 1920,
    //     //         h: 1080,
    //     //     },
    //     //     &json::json!({}),
    //     // );
    //     i += 1;
    // }
    println!("{}", now.elapsed().as_secs_f64())
}

#[allow(dead_code)]
fn test_ffmpeg_cmd() -> Result<()> {
    // let (mut reader, writer) = os_pipe::pipe()?;
    // let a = cmd!(
    //     "ffmpeg",
    //     "-hide_banner",
    //     "-ss",
    //     "0",
    //     "-i",
    //     "1.mp4",
    //     "-f",
    //     "rawvideo",
    //     "-frames:v",
    //     "1",
    //     "pipe:"
    // )
    // .std
    // .run()?
    // .stdout;
    // let a = String::from_utf8_lossy(&a);
    // let mut ss = String::new();
    // reader.read_to_string(&mut ss)?;
    // .pipe(cmd!("echo"))
    // .stdout_capture()

    //command.stdout(Stdio::null());

    //handle.wait()?;
    //reader.read_to_string(&mut out);
    //println!("{}", out);
    //println!("{}", a);
    //println!("{}", ss);
    Ok(())
}
