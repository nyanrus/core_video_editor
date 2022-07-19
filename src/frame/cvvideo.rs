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

use super::frame::*;
use std::{mem::ManuallyDrop, rc::Weak};
use std::rc::Rc;

use opencv::{prelude::*, videoio::{VideoCapture, CAP_FFMPEG, VideoWriter, CAP_PROP_FRAME_COUNT, CAP_PROP_POS_FRAMES, VideoWriterProperties, VideoCaptureProperties, CAP_PROP_HW_ACCELERATION, VIDEO_ACCELERATION_D3D11, CAP_PROP_HW_ACCELERATION_USE_OPENCL, VIDEO_ACCELERATION_ANY, VIDEO_ACCELERATION_MFX, VIDEO_ACCELERATION_VAAPI, VIDEO_ACCELERATION_NONE, CAP_PROP_BUFFERSIZE}, Error, core::{Vector, USAGE_DEFAULT, UMat, UMatUsageFlags, Scalar_, BORDER_TRANSPARENT, ToInputArray}, calib3d::USAC_DEFAULT, imgproc::WARP_POLAR_LINEAR};
pub struct FrameSize {
    width:i32,
    height:i32,
}

pub struct VideoWriterSetting {
    file_name:String,
    api_preference:i32,
    fourcc:i32,
    fps:f64,
    frame_size: FrameSize,
    is_color:bool,
}

pub fn get_video_capture(file_name:&str) -> Result<VideoCapture,Error>{
    // return VideoCapture::from_file(file_name, CAP_FFMPEG);
    return VideoCapture::from_file_with_params(file_name, CAP_FFMPEG,&Vector::from_iter(
        [
            // CAP_PROP_HW_ACCELERATION,
            // VIDEO_ACCELERATION_D3D11,
            CAP_PROP_HW_ACCELERATION_USE_OPENCL,
            1
        ]
    ));
}

pub fn get_video_writer(settings:VideoWriterSetting)->Result<VideoWriter,Error> {
    return VideoWriter::new_with_backend(
        &settings.file_name,
        settings.api_preference,
        settings.fourcc,
        settings.fps,
        opencv::core::Size {
            width:settings.frame_size.width,
            height:settings.frame_size.height,
        },
        settings.is_color
    );
}

struct CvFrame {
    f:Frame,
}
impl FrameInterface for CvFrame{
    fn process_frame(&self,f:Option<&Frame>) -> Result<Option<&Frame>, std::fmt::Error> {
        todo!()
    }
}

fn a(){
    let mut vec = Vec::<Box<dyn FrameInterface>>::new();
    let a = CvFrame{f:Frame{ w: 1, h: 1, pix_vec: Vec::new() }};
    vec.push(Box::new(a) as Box<dyn FrameInterface>);
    for i in vec {
        let a = FrameInterface::process_frame(i.as_ref(), None);
    }
}

pub fn get_video_frame(vc:&mut VideoCapture, frame_num:f64) -> UMat{
    //if frame_num != vc.get(CAP_PROP_POS_FRAMES).unwrap() {
    vc.set(CAP_PROP_BUFFERSIZE,2.0).unwrap();
    vc.set(CAP_PROP_POS_FRAMES, frame_num).unwrap();
    //}
    // check if you needed is vc reach to end,
    // use Mat::empty()
    //let mut frame = Mat::default();
    let mut umat =UMat::new(UMatUsageFlags::USAGE_DEFAULT);
    //let mut umat = frame.get_umat(opencv::core::AccessFlag::ACCESS_FAST, opencv::core::UMatUsageFlags::USAGE_DEFAULT).unwrap();
    vc.retrieve(&mut umat, 0).unwrap();
    //vc.read(&mut umat).unwrap();
    return umat;
}

// pub fn blend_frame(src:&UMat,dst:&UMat,alpha:f64) -> Result<UMat,Error>{
//     let mut umat = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
//     opencv::core::add_weighted(&src, alpha, &dst, 1.-alpha, 0., &mut umat, opencv::core::CV_8UC4)?;
//     return Ok(umat);
// }

pub fn warp_affine(src:&UMat,dst:&mut UMat, M:&dyn ToInputArray) {
    let dsize = dst.size().unwrap();
    opencv::imgproc::warp_affine(src, dst, M, dsize, WARP_POLAR_LINEAR, BORDER_TRANSPARENT, Scalar_::new(0.,0.,0.,0.)).unwrap();
}
