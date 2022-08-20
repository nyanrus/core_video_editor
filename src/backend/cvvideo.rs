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

use std::ffi::c_void;
use std::sync::Mutex;

use crate::io::input::InputInterface;
use crate::io::output::OutputInterface;
use opencv::imgproc::COLOR_BGR2RGBA;
use rgb::FromSlice;
use serde_json as json;

use rayon::prelude::*;

use opencv as cv;
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
    fn in_open_file(&self, file: &str) -> Option<Box<dyn FrameInterface>> {
        match get_video_capture(file) {
            Ok(mut o) => {
                o.set(CAP_PROP_BUFFERSIZE, 2.0).unwrap();
                Some(Box::new(CvFrameIn {
                    id: Ulid::new(),
                    vc: Mutex::new(o),
                }) as Box<dyn FrameInterface>)
            }
            Err(_) => None,
        }
    }
}

impl OutputInterface for IOpenCV {
    fn out_open_file(&self, file: &str) -> Option<Box<dyn FrameInterface>> {
        match get_video_writer(VideoWriterSetting {
            file_name: file.to_string(),
            fourcc: u32::from_ne_bytes(*(b"h264" as &[u8; 4])) as i32,
            fps: 30.,
            frame_size: FrameSize {
                width: 1920,
                height: 1080,
            },
            is_color: true,
        }) {
            Ok(o) => Some(Box::new(CvFrameOut {
                id: Ulid::new(),
                vw: Mutex::new(o),
            }) as Box<dyn FrameInterface>),
            Err(_) => None,
        }
    }
}

pub struct VideoWriterSetting {
    file_name: String,
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
        CAP_FFMPEG,
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

    fn process(&self, f: &mut Frame, settings: &Settings, json: &json::Value) -> bool {
        //println!("{:?}", json);
        let frame = get_video_frame(&self.vc, settings.frame_num as f64);
        // println!("{}", settings.frame_num as f64);
        // println!("{}", frame.empty());
        if frame.empty() {
            return false;
        }
        let mut mat = Mat::default();
        opencv::imgproc::cvt_color(&frame, &mut mat, COLOR_BGR2RGBA, 4).unwrap();
        let arr_frame = mat.data_bytes().unwrap();
        // println!("{}", arr_frame.len());
        f.vec_rgba = arr_frame
            .par_iter()
            .map(|x| *x as f32 / 255.)
            .collect::<Vec<f32>>()
            .as_rgba()
            .to_owned();
        // println!("{}", f.vec_rgba.len());
        true
    }

    fn get_ulid(&self) -> ulid::Ulid {
        self.id
    }
}

pub fn get_video_frame(vc: &Mutex<VideoCapture>, frame_num: f64) -> Mat {
    //if frame_num != vc.get(CAP_PROP_POS_FRAMES).unwrap() {
    let mut mvc = vc.lock().unwrap();
    mvc.set(CAP_PROP_POS_FRAMES, frame_num).unwrap();
    //}
    // check if you needed is vc reach to end,
    // use Mat::empty()
    let mut frame = Mat::default();
    if mvc.get(CAP_PROP_POS_FRAMES).unwrap() != frame_num {
        return frame;
    }
    //let mut umat = frame.get_umat(opencv::core::AccessFlag::ACCESS_FAST, opencv::core::UMatUsageFlags::USAGE_DEFAULT).unwrap();
    mvc.retrieve(&mut frame, 0).unwrap();

    // println!("aaa{}", frame.data_bytes().unwrap().len());
    //vc.read(&mut umat).unwrap();
    frame
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

use crate::base::frame::{Frame, FrameInterface, Settings};

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

struct CvFrameOut {
    pub id: Ulid,
    pub vw: Mutex<VideoWriter>,
}

impl FrameInterface for CvFrameOut {
    fn get_settings(&self) -> json::Value {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&self, f: &mut Frame, settings: &Settings, json: &json::Value) -> bool {
        // println!("{}", f.vec_rgba.len());
        let mut v = f
            .vec_rgba
            .par_iter()
            .map(|x| [(x.b * 255.) as u8, (x.g * 255.) as u8, (x.r * 255.) as u8])
            .flatten_iter()
            .collect::<Vec<u8>>();
        //println!("{:?}", v);

        //let b = Mat::from_slice(&v).unwrap();
        // let mut dat = get_video_frame(&Mutex::new(get_video_capture("1.mp4").unwrap()), 0.)
        //     .data_bytes()
        //     .unwrap()
        //     .to_vec();

        let b = unsafe {
            Mat::new_size_with_data(
                cv::core::Size_ {
                    width: f.w as i32,
                    height: f.h as i32,
                },
                cv::core::CV_8UC3,
                v.as_mut_ptr() as *mut c_void,
                cv::core::Mat_AUTO_STEP,
            )
        }
        .unwrap();
        // let b = Mat::from_exact_iter(

        //         .iter()
        //         .map(|x| *x),
        // )
        // .unwrap();
        // let b = Mat::from_slice::<cv::core::VecN<u8, 3>>(

        //         .par_iter()
        //         .chunks(3)
        //         .map(|x| cv::core::VecN::<u8, 3>::from([*x[0], *x[1], *x[2]]))
        //         .collect::<Vec<cv::core::VecN<u8, 3>>>(),
        // )
        // .unwrap();

        // let test = b.data_bytes().unwrap();

        // println!("{}", b.empty());

        self.vw.lock().unwrap().write(&b).unwrap();
        //self.vw.lock().unwrap().release().unwrap();
        true
    }
}

impl Drop for CvFrameOut {
    fn drop(&mut self) {
        let mut a = self.vw.lock().unwrap();
        a.release().unwrap();
    }
}
