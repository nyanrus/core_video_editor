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

use crate::base::frame::Frame;


use image::Rgba32FImage;

use imageproc::geometric_transformations as geo_transform;

use rayon::prelude::*;


pub fn a(a: &Frame, b: &Frame) {
    let a = Rgba32FImage::from_vec(
        1920,
        1080,
        a.vec_rgba
            .par_iter()
            .map(|x| [x.r, x.g, x.b, x.a])
            .flatten_iter()
            .collect(),
    )
    .unwrap();
    let img = geo_transform::warp(
        &a,
        &geo_transform::Projection::scale(0.5, 0.5),
        geo_transform::Interpolation::Nearest,
        image::Rgba::<f32>([0., 0., 0., 0.]),
    );
}
