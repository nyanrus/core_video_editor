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

use anyhow::{anyhow, Result};
use std::sync::Mutex;

use serde_json as json;

use ffmpeg::ffi::{avformat_seek_file, AVSEEK_FLAG_BACKWARD};
use ffmpeg::format::{self, Pixel};
use ffmpeg::frame::Audio;
use ffmpeg::frame::Video;
use ffmpeg::media::Type;
use ffmpeg::{software::*, ChannelLayout};
use ffmpeg::{Rational, Stream};
use ffmpeg_next as ffmpeg;

use crate::base::frame::FrameSettings;
use crate::base::{frame::Frame, interface::ProcessInterface};
use crate::io::input::InputInterface;

pub mod read;
pub mod util;

use util::*;

pub struct FFVideo {
    pub last_pts: i64,
    pub tb: Rational,
    pub afr: Rational,
    pub index: usize,
    pub decoder: ffmpeg::decoder::Video,
    pub scaler: scaling::Context,
}

pub struct FFAudio {
    pub last_pts: i64,
    pub tb: Rational,
    pub sample: u32,
    pub index: usize,
    pub decoder: ffmpeg::decoder::Audio,
    pub resampler: resampling::Context,
}

pub enum FFInputChild {
    Video(FFVideo),
    Audio(FFAudio),
}

pub struct FFInput {
    pub ctx: Mutex<format::context::Input>,
    pub children: Vec<FFInputChild>,
    pub best_va: (usize, usize),
}

impl FFInput {
    pub fn init(path: &str) -> Result<Self> {
        match format::input(&path) {
            Ok(input) => {
                let mut vec = Vec::<FFInputChild>::new();
                let bbb = input.streams().collect::<Vec<Stream>>();
                for i in bbb {
                    if i.parameters().medium() == Type::Video {
                        let context_decoder =
                            ffmpeg::codec::context::Context::from_parameters(i.parameters())?;
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

                        vec.push(FFInputChild::Video(FFVideo {
                            last_pts: -1,
                            tb: i.time_base(),
                            afr: i.avg_frame_rate(),
                            index: i.index(),
                            decoder,
                            scaler,
                        }));
                    } else if i.parameters().medium() == Type::Audio {
                        let context_decoder =
                            ffmpeg::codec::context::Context::from_parameters(i.parameters())?;
                        let decoder = context_decoder.decoder().audio()?;
                        let resampler = resampler(
                            (decoder.format(), decoder.channel_layout(), decoder.rate()),
                            (
                                format::Sample::F32(format::sample::Type::Planar),
                                ChannelLayout::STEREO,
                                48000,
                            ),
                        )?;
                        vec.push(FFInputChild::Audio(FFAudio {
                            last_pts: -1,
                            tb: i.time_base(),
                            sample: decoder.rate(),
                            index: i.index(),
                            decoder,
                            resampler,
                        }))
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
                    children: vec,
                    best_va: (i_vb, i_ab),
                })
            }
            Err(_) => todo!(),
        }
    }
}

impl ProcessInterface<Frame, FrameSettings> for FFInput {
    fn get_ulid(&self) -> ulid::Ulid {
        ulid::Ulid::new()
    }

    fn process(&mut self, f: &mut Frame, settings: &FrameSettings, _json: json::Value) -> bool {
        let fps = match &self.children[self.best_va.0] {
            FFInputChild::Video(v) => v.afr,
            FFInputChild::Audio(_) => todo!(),
        };
        let fps = fps.0 as f64 / fps.1 as f64;
        read::read_video(
            self,
            self.best_va.0,
            frame_num2time(settings.frame_num as u32, fps),
            f,
        )
        .unwrap();
        true
    }

    fn get_json_template(&self) -> serde_json::Value {
        todo!()
    }
}
pub struct FFInit();

impl InputInterface<Frame, FrameSettings> for FFInit {
    fn in_open_file(&self, _file: &str) -> Option<Box<dyn ProcessInterface<Frame, FrameSettings>>> {
        match FFInput::init(_file) {
            Ok(o) => Some(Box::new(o) as Box<dyn ProcessInterface<Frame, FrameSettings>>),
            Err(_e) => None,
        }
    }
}
