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

use std::collections::BTreeMap;

use anyhow::Result;
use ffmpeg::frame::{Audio, Video};
use parking_lot::Mutex;

use serde_json as json;

use crate::base::frame::{Frame, FrameSettings};
use crate::base::interface::ProcessInterface;

use super::util::*;
use ffmpeg::format::{self, Pixel};
use ffmpeg::media::Type;
use ffmpeg::{software::*, ChannelLayout};
use ffmpeg::{Rational, Stream};
use ffmpeg_next as ffmpeg;

pub struct FFVideo {
    pub last_pts: i64,
    pub time_base: Rational,
    pub afr: Rational,
    pub index: usize,
    pub decoder: ffmpeg::decoder::Video,
    pub scaler: scaling::Context,
    pub cache_buf: BTreeMap<i64, Video>,
}

pub struct FFAudio {
    pub last_pts: i64,
    pub time_base: Rational,
    pub sample_rate: u32,
    pub index: usize,
    pub decoder: ffmpeg::decoder::Audio,
    pub resampler: resampling::Context,
    pub cache_buf: BTreeMap<i64, Audio>,
}

pub enum FFInputChild {
    Video(FFVideo),
    Audio(FFAudio),
}

pub struct FFInput {
    pub ctx: Mutex<format::context::Input>,
    pub last_pts: i64,
    pub children: Vec<FFInputChild>,
    pub best_va: (usize, usize),
}

unsafe impl Send for FFInput {}

impl FFVideo {
    pub fn init(i: Stream) -> Result<Self> {
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(i.parameters())?;
        let decoder = context_decoder.decoder().video()?;
        let scaler = scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGBA,
            decoder.width(),
            decoder.height(),
            scaling::flag::Flags::BILINEAR,
        )?;
        Ok(FFVideo {
            last_pts: -1,
            time_base: i.time_base(),
            afr: i.avg_frame_rate(),
            index: i.index(),
            decoder,
            scaler,
            cache_buf: BTreeMap::new(),
        })
    }
}

impl FFAudio {
    pub fn init(i: Stream) -> Result<Self> {
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(i.parameters())?;
        let decoder = context_decoder.decoder().audio()?;
        let resampler = resampler(
            (decoder.format(), decoder.channel_layout(), decoder.rate()),
            (
                format::Sample::F32(format::sample::Type::Packed),
                ChannelLayout::STEREO,
                decoder.rate(),
            ),
        )?;
        Ok(FFAudio {
            last_pts: -1,
            time_base: i.time_base(),
            sample_rate: decoder.rate(),
            index: i.index(),
            decoder,
            resampler,
            cache_buf: BTreeMap::new(),
        })
    }
}

impl FFInput {
    pub fn init(path: &str) -> Result<Self> {
        match format::input(&path) {
            Ok(input) => {
                let mut vec = Vec::<FFInputChild>::new();
                let bbb = input.streams().collect::<Vec<Stream>>();
                for i in bbb {
                    if i.parameters().medium() == Type::Video {
                        vec.push(FFInputChild::Video(FFVideo::init(i)?));
                    } else if i.parameters().medium() == Type::Audio {
                        vec.push(FFInputChild::Audio(FFAudio::init(i)?))
                    }
                }
                let vb = input.streams().best(Type::Video).unwrap();
                let ab = input.streams().best(Type::Audio).unwrap();

                let (mut i_vb, mut i_ab) = (0, 0);
                for (i, s) in vec.iter().enumerate() {
                    match s {
                        FFInputChild::Video(v) => {
                            if v.index == vb.index() {
                                i_vb = i;
                            }
                        }
                        FFInputChild::Audio(a) => {
                            if a.index == ab.index() {
                                i_ab = i;
                            }
                        }
                    }
                }

                Ok(FFInput {
                    ctx: Mutex::new(input),
                    last_pts: -1,
                    children: vec,
                    best_va: (i_vb, i_ab),
                })
            }
            Err(_) => todo!(),
        }
    }

    pub fn get_video_elem(&mut self, index: usize) -> Option<&mut FFVideo> {
        if let Some(FFInputChild::Video(v)) = self.children.get_mut(index) {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_audio_elem(&mut self, index: usize) -> Option<&mut FFAudio> {
        if let Some(FFInputChild::Audio(a)) = self.children.get_mut(index) {
            Some(a)
        } else {
            None
        }
    }
}

impl ProcessInterface<Frame, FrameSettings> for FFInput {
    fn get_ulid(&self) -> ulid::Ulid {
        ulid::Ulid::new()
    }

    fn process(
        &mut self,
        f: &mut Box<Frame>,
        settings: &FrameSettings,
        _json: json::Value,
    ) -> bool {
        let v = match &mut self.children[self.best_va.0] {
            FFInputChild::Video(v) => v,
            FFInputChild::Audio(_) => todo!(),
        };
        let fps = v.afr;
        let fps = fps.0 as f64 / fps.1 as f64;
        let mut input = self.ctx.lock();
        v.read_video(
            &mut input,
            frame_num2time(settings.frame_num as u32, fps),
            f,
        );
        //.unwrap();
        true
    }

    fn get_json_template(&self) -> serde_json::Value {
        todo!()
    }
}

impl ProcessInterface<Vec<f32>, FrameSettings> for FFInput {
    fn get_ulid(&self) -> ulid::Ulid {
        ulid::Ulid::new()
    }

    fn process(
        &mut self,
        f: &mut Box<Vec<f32>>,
        settings: &FrameSettings,
        _json: json::Value,
    ) -> bool {
        let fps = match &mut self.children[self.best_va.0] {
            FFInputChild::Video(v) => v.afr,
            FFInputChild::Audio(_) => todo!(),
        };
        let a = match &mut self.children[self.best_va.1] {
            FFInputChild::Video(_) => todo!(),
            FFInputChild::Audio(a) => a,
        };
        //let fps = v.afr;
        let fps = fps.0 as f64 / fps.1 as f64;
        //println!("fps: {}", fps);
        let mut input = self.ctx.lock();
        a.read_audio(
            &mut input,
            frame_num2time(settings.frame_num as u32, fps),
            f,
            frame_num2time(1, fps),
        );
        //.unwrap();
        true
    }

    fn get_json_template(&self) -> serde_json::Value {
        todo!()
    }
}
