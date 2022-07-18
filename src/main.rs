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

use std::{env, time, io::Write};

use opencv::{
	core::{self, UMat, UMatUsageFlags, Vector, Point2f, USAGE_DEFAULT},
	imgcodecs,
	imgproc,
	prelude::*,
	Result,
	types, videoio::CAP_PROP_FRAME_COUNT,
};
opencv::opencv_branch_4! {
	use opencv::core::AccessFlag::ACCESS_READ;
}
opencv::not_opencv_branch_4! {
	use opencv::core::ACCESS_READ;
}
use std::time::{Duration, Instant};

mod frame;
use frame::cvvideo;
use frame::cvvideo::*;

//const ITERATIONS: usize =1000;

fn main() {
    core::set_use_opencl(true).unwrap();
    let mut vc_ref = cvvideo::get_video_capture("anim.mp4").unwrap();
    println!("{}",vc_ref.get(CAP_PROP_FRAME_COUNT).unwrap());
	let a = cvvideo::get_video_frame(&mut vc_ref,1.0);
	let mut b = cvvideo::get_video_frame(&mut vc_ref,2.0);
    let now = Instant::now();
	println!("");
    for c in 1..1000 {
		print!("\r{}",c);
		std::io::stdout().flush().unwrap();
        warp_affine(&a, &mut b, &Mat::from_slice_2d(&[[1.,0.,0.],[0.,1.,0.]]).unwrap());//.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT).unwrap());
    }
	println!("");
    println!("{}ms", now.elapsed().as_millis());
    //println!("{}",a.empty());
    //println!("{:?}",a);
}

// fn main() -> Result<()> {
// 	let img_file = env::args().nth(1).expect("Please supply image file name");
// 	let opencl_have = core::have_opencl()?;
// 	if opencl_have {
// 		core::set_use_opencl(true)?;
// 		let mut platforms = types::VectorOfPlatformInfo::new();
// 		core::get_platfoms_info(&mut platforms)?;
// 		for (platf_num, platform) in platforms.into_iter().enumerate() {
// 			println!("Platform #{}: {}", platf_num, platform.name()?);
// 			for dev_num in 0..platform.device_number()? {
// 				let mut dev = core::Device::default();
// 				platform.get_device(&mut dev, dev_num)?;
// 				println!("  OpenCL device #{}: {}", dev_num, dev.name()?);
// 				println!("    vendor:  {}", dev.vendor_name()?);
// 				println!("    version: {}", dev.version()?);
// 			}
// 		}
// 	}
// 	let opencl_use = core::use_opencl()?;
// 	println!(
// 		"OpenCL is {} and {}",
// 		if opencl_have { "available" } else { "not available" },
// 		if opencl_use { "enabled" } else { "disabled" },
// 	);
// 	// println!("Timing CPU implementation... ");
// 	// let img = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;
// 	// let start = time::Instant::now();
// 	// for _ in 0..ITERATIONS {
// 	// 	let mut gray = Mat::default();
// 	// 	imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
// 	// 	let mut blurred = Mat::default();
// 	// 	imgproc::gaussian_blur(&gray, &mut blurred, core::Size::new(7, 7), 1.5, 0., core::BORDER_DEFAULT)?;
// 	// 	let mut edges = Mat::default();
// 	// 	imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
// 	// }
// 	// println!("{:#?}", start.elapsed());
// 	if opencl_use {
// 		println!("Timing OpenCL implementation... ");
// 		let mat = imgcodecs::imread(&img_file, imgcodecs::IMREAD_COLOR)?;
// 		let img = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
// 		let start = time::Instant::now();
// 		for _ in 0..ITERATIONS {
// 			let mut gray = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
// 			imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
// 			let mut blurred = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
// 			imgproc::gaussian_blur(&gray, &mut blurred, core::Size::new(7, 7), 1.5, 0., core::BORDER_DEFAULT)?;
// 			let mut edges = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
// 			imgproc::canny(&blurred, &mut edges, 0., 50., 3, false)?;
//             let result = edges.get_mat(ACCESS_READ ).unwrap();
// 		}
// 		println!("{:#?}", start.elapsed());
// 	}
// 	Ok(())
// }