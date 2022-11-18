use anyhow::{anyhow, Result};

use std::sync::Mutex;

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

use serde_json as json;

use crate::io::input::InputInterface;

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

fn ts2time<T: Into<NRRational>>(pts: i64, time_base: T) -> f64 {
    let time_base = time_base.into();
    pts as f64 * time_base.0 as f64 / time_base.1 as f64
}

fn time2ts<T: Into<NRRational>>(time: f64, time_base: T) -> i64 {
    let time_base = time_base.into();
    (time * time_base.1 as f64 / time_base.0 as f64).round() as i64
}

fn frame_num2time(frame_num: u32, fps: f64) -> f64 {
    frame_num as f64 / fps
}

fn time2frame_num(time: f64, fps: f64) -> u32 {
    (time * fps).round() as u32
}

fn ts2frame_num<T: Into<NRRational>>(pts: i64, time_base: T, fps: f64) -> u32 {
    time2frame_num(ts2time(pts, time_base), fps)
}

fn frame_num2ts<T: Into<NRRational>>(frame_num: u32, time_base: T, fps: f64) -> i64 {
    time2ts(frame_num2time(frame_num, fps), time_base)
}

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
                        let scaler = scaler(
                            Pixel::RGBA,
                            scaling::flag::Flags::BILINEAR,
                            (decoder.width(), decoder.height()),
                            (decoder.width(), decoder.height()),
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

pub mod read {
    use ffmpeg::ffi::AVFormatContext;

    use super::*;
    pub fn read_video(
        ctx: &mut FFInput,
        index: usize,
        time: f64,
        frame: &mut Frame,
    ) -> Result<bool> {
        let o = ctx.ctx.get_mut().unwrap();
        let video = match ctx
            .children
            .get_mut(index)
            .ok_or_else(|| anyhow!("index is not valid"))?
        {
            FFInputChild::Video(v) => v,
            FFInputChild::Audio(_) => return Err(anyhow!("audio index in read_video")),
        };
        let video_stream_index = video.index;

        let avg_frame_rate = video.afr.0 as f64 / video.afr.1 as f64;
        let pts = time2ts(time, (video.tb.0, video.tb.1));

        println!(
            "pts: {} , pts->f_num: {}",
            pts,
            ts2frame_num(pts, (video.tb.0, video.tb.1), avg_frame_rate)
        );

        //if requested timestamp <= last timestamp that read
        if video.last_pts >= pts {
            println!("seek");
            video.decoder.flush();
            unsafe {
                seek(o.as_mut_ptr(), video_stream_index as i32, pts)?;
            }
        }

        let pkts = o.packets();
        let mut v = Video::empty();
        let decoder = &mut video.decoder;
        let mut b = read_raw_frame(&mut v, None, decoder, true, pts)?;

        if !b {
            'outer: for i in pkts {
                match i.0.index().cmp(&video_stream_index) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => {
                        b = read_raw_frame(&mut v, Some(&i.1), decoder, false, pts)?;
                        if b {
                            break 'outer;
                        }
                    }
                    std::cmp::Ordering::Greater => return Ok(false),
                }
            }
        }

        if !b {
            b = read_raw_frame(&mut v, None, decoder, false, pts)?;
        }

        if !b {
            decoder.flush();
            panic!("irregular")
        }
        let mut vid = Video::empty();

        video.scaler.run(&v, &mut vid)?;

        frame.vec_rgba = vid.data(0).to_vec();

        println!("end");
        Ok(true)
    }

    pub fn read_audio(
        ctx: &mut FFInput,
        index: usize,
        time: f64,
        samples: &mut Vec<f32>,
    ) -> Result<bool> {
        println!("audio");
        let mut a = Audio::empty();
        let input = ctx.ctx.get_mut().unwrap();

        let audio = match ctx
            .children
            .get_mut(index)
            .ok_or_else(|| anyhow!("index is not valid"))?
        {
            FFInputChild::Video(_) => return Err(anyhow!("audio index in read_video")),
            FFInputChild::Audio(a) => a,
        };

        let pts = time2ts(time, audio.tb);

        println!("audio_tb : {}", audio.tb);

        let mut b = read_raw_frame(&mut a, None, &mut audio.decoder, true, pts)?;

        if !b {
            'outer: for (s, p) in input.packets() {
                if s.index() == audio.index {
                    match s.index().cmp(&audio.index) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => {
                            b = read_raw_frame(&mut a, Some(&p), &mut audio.decoder, false, 48000)?;
                            if b {
                                break 'outer;
                            }
                        }
                        std::cmp::Ordering::Greater => return Ok(false),
                    }
                }
            }
        }

        if !b {
            b = read_raw_frame(&mut a, None, &mut audio.decoder, false, pts)?;
        }

        if !b {
            audio.decoder.flush();
            panic!("irregular")
        }

        let mut aud = Audio::empty();
        audio.resampler.run(&a, &mut aud)?;

        *samples = bytemuck::cast_slice(aud.data(0)).to_vec();
        Ok(true)
    }

    unsafe fn seek(ctx_ptr: *mut AVFormatContext, idx: i32, ts: i64) -> Result<()> {
        match avformat_seek_file(ctx_ptr, idx, i64::min_value(), ts, ts, AVSEEK_FLAG_BACKWARD) {
            0.. => Ok(()),
            err => Err(anyhow::anyhow!("seek_error : {}", err)),
        }
    }

    pub fn read_raw_frame(
        frame: &mut ffmpeg::Frame,
        packet: Option<&ffmpeg::Packet>,
        decoder: &mut ffmpeg::codec::decoder::Opened,
        read_only: bool,
        ts: i64,
    ) -> Result<bool> {
        if !read_only {
            match packet {
                Some(s) => decoder.send_packet(s)?,
                None => decoder.send_eof()?,
            };
        }

        while decoder.receive_frame(frame).is_ok() {
            println!("{:?}", frame.pts());
            if frame.pts() == Some(ts) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn get_fps(ctx: &mut FFInput, index: usize) -> NRRational {
        match &ctx.children[index] {
            FFInputChild::Video(v) => v.afr.into(),
            FFInputChild::Audio(_) => todo!(),
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
struct FFInit();

impl InputInterface<Frame, FrameSettings> for FFInit {
    fn in_open_file(&self, _file: &str) -> Option<Box<dyn ProcessInterface<Frame, FrameSettings>>> {
        match FFInput::init(_file) {
            Ok(o) => Some(Box::new(o) as Box<dyn ProcessInterface<Frame, FrameSettings>>),
            Err(_e) => None,
        }
    }
}
