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

use std::sync::Mutex;

use crate::io::input::InputInterface;
use opencv::imgproc::COLOR_BGR2RGBA;
use rgb::FromSlice;
use serde_json as json;

use crate::frame::*;

use rayon::prelude::*;

use opencv::core::AccessFlag;
use opencv::{
    core::{Scalar_, ToInputArray, UMat, UMatUsageFlags, Vector, BORDER_TRANSPARENT},
    imgproc::WARP_POLAR_LINEAR,
    prelude::*,
    videoio::{
        VideoCapture, VideoWriter, CAP_FFMPEG, CAP_PROP_BUFFERSIZE,
        CAP_PROP_HW_ACCELERATION_USE_OPENCL, CAP_PROP_POS_FRAMES,
    },
    Error,
};
use ulid::Ulid;
pub struct FrameSize {
    width: i32,
    height: i32,
}

pub struct IOpenCV {}

impl InputInterface for IOpenCV {
    fn open_file(&self, file: &str) -> Option<Box<dyn FrameInterface>> {
        match get_video_capture(file) {
            Ok(o) => Some(Box::new(CvFrameIn {
                id: Ulid::new(),
                vc: Mutex::new(o),
            }) as Box<dyn FrameInterface>),
            Err(_) => None,
        }
    }
}

pub struct VideoWriterSetting {
    file_name: String,
    api_preference: i32,
    fourcc: i32,
    fps: f64,
    frame_size: FrameSize,
    is_color: bool,
}

pub fn get_video_capture(file_name: &str) -> Result<VideoCapture, Error> {
    // return VideoCapture::from_file(file_name, CAP_FFMPEG);
    VideoCapture::from_file_with_params(
        file_name,
        CAP_FFMPEG,
        &Vector::from_iter([
            // CAP_PROP_HW_ACCELERATION,
            // VIDEO_ACCELERATION_D3D11,
            CAP_PROP_HW_ACCELERATION_USE_OPENCL,
            1,
        ]),
    )
}

pub fn get_video_writer(settings: VideoWriterSetting) -> Result<VideoWriter, Error> {
    VideoWriter::new_with_backend(
        &settings.file_name,
        settings.api_preference,
        settings.fourcc,
        settings.fps,
        opencv::core::Size {
            width: settings.frame_size.width,
            height: settings.frame_size.height,
        },
        settings.is_color,
    )
}

pub struct CvFrameIn {
    pub id: Ulid,
    pub vc: Mutex<VideoCapture>,
}

impl FrameInterface for CvFrameIn {
    fn get_settings(&self) -> json::Value {
        json::json!("{'frame_num':0}")
    }

    fn process(&self, f: &mut Frame, json: &json::Value) -> bool {
        println!("{:?}", json);
        let frame = get_video_frame(&self.vc, json["frame_num"].as_u64().unwrap() as f64);
        let mut umat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        opencv::imgproc::cvt_color(&frame, &mut umat, COLOR_BGR2RGBA, 4).unwrap();
        let mat_frame = umat.get_mat(AccessFlag::ACCESS_READ).unwrap();
        let arr_frame = mat_frame.data_bytes().unwrap();
        f.vec_rgba = arr_frame
            .par_iter()
            .map(|x| *x as f32 / 255.)
            .collect::<Vec<f32>>()
            .as_rgba()
            .to_owned();
        true
    }

    fn get_ulid(&self) -> ulid::Ulid {
        self.id
    }
}

pub fn get_video_frame(vc: &Mutex<VideoCapture>, frame_num: f64) -> UMat {
    //if frame_num != vc.get(CAP_PROP_POS_FRAMES).unwrap() {
    let mut mvc = vc.lock().unwrap();
    mvc.set(CAP_PROP_BUFFERSIZE, 2.0).unwrap();
    mvc.set(CAP_PROP_POS_FRAMES, frame_num).unwrap();
    //}
    // check if you needed is vc reach to end,
    // use Mat::empty()
    //let mut frame = Mat::default();
    let mut umat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
    //let mut umat = frame.get_umat(opencv::core::AccessFlag::ACCESS_FAST, opencv::core::UMatUsageFlags::USAGE_DEFAULT).unwrap();
    mvc.retrieve(&mut umat, 0).unwrap();
    //vc.read(&mut umat).unwrap();
    umat
}

// pub fn blend_frame(src:&UMat,dst:&UMat,alpha:f64) -> Result<UMat,Error>{
//     let mut umat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
//     opencv::core::add_weighted(&src, alpha, &dst, 1.-alpha, 0., &mut umat, opencv::core::CV_8UC4)?;
//     return Ok(umat);
// }

pub fn warp_affine(src: &UMat, dst: &mut UMat, m: &dyn ToInputArray) {
    let dsize = dst.size().unwrap();

    opencv::imgproc::warp_affine(
        src,
        dst,
        m,
        dsize,
        WARP_POLAR_LINEAR,
        BORDER_TRANSPARENT,
        Scalar_::new(0., 0., 0., 0.),
    )
    .unwrap();
}

use crate::frame::frame::{Frame, FrameInterface};

pub async fn warp_and_blend(src: &Frame, dst: &mut Frame) {
    let s_rgba = src.vec_rgba.par_iter();
    let d_rgba = dst.vec_rgba.par_iter_mut();

    s_rgba
        .zip(d_rgba)
        .map(|(s_rgba, d_rgba)| {
            if s_rgba.a == 0. {
            } else if s_rgba.a == 1. {
                *d_rgba = *s_rgba;
            } else {
                d_rgba.r = s_rgba.r * s_rgba.a + d_rgba.r * d_rgba.a * (1. - s_rgba.a);
                d_rgba.g = s_rgba.g * s_rgba.a + d_rgba.g * d_rgba.a * (1. - s_rgba.a);
                d_rgba.b = s_rgba.b * s_rgba.a + d_rgba.b * d_rgba.a * (1. - s_rgba.a);
                d_rgba.a *= 1. - s_rgba.a;
            }
        })
        .collect::<()>();
    //futures::future::join_all(tasks).await;
    // dst.vec_rgba=d_rgb.par_iter().zip(d_a.par_iter()).map(|(x,y)|{
    //     [x[0],x[1],x[2],*y]
    // }).collect();
    //warp_affine(src, dst, m);
}
