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
use ffmpeg::Rational;
use ffmpeg_next as ffmpeg;

pub struct NRRational(i32, i32);

impl From<Rational> for NRRational {
    fn from(value: Rational) -> Self {
        NRRational(value.0, value.1)
    }
}

impl From<(i32, i32)> for NRRational {
    fn from(val: (i32, i32)) -> Self {
        NRRational(val.0, val.1)
    }
}

impl From<NRRational> for f64 {
    fn from(value: NRRational) -> Self {
        value.0 as f64 / value.1 as f64
    }
}

pub fn ts2time<T: Into<NRRational>>(pts: i64, time_base: T) -> f64 {
    let time_base = time_base.into();
    pts as f64 * time_base.0 as f64 / time_base.1 as f64
}

pub fn time2ts<T: Into<NRRational>>(time: f64, time_base: T) -> i64 {
    let time_base = time_base.into();
    (time * time_base.1 as f64 / time_base.0 as f64).round() as i64
}

pub fn frame_num2time(frame_num: u32, fps: f64) -> f64 {
    frame_num as f64 / fps
}

pub fn time2frame_num(time: f64, fps: f64) -> u32 {
    (time * fps).round() as u32
}

pub fn ts2frame_num<T: Into<NRRational>>(pts: i64, time_base: T, fps: f64) -> u32 {
    time2frame_num(ts2time(pts, time_base), fps)
}

pub fn frame_num2ts<T: Into<NRRational>>(frame_num: u32, time_base: T, fps: f64) -> i64 {
    time2ts(frame_num2time(frame_num, fps), time_base)
}
