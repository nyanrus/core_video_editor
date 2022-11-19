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

use std::cmp::Ordering;

use crate::backend::ffmpeg::FFInputChild;
use crate::base::frame::Frame;

use anyhow::{anyhow, Result};

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use super::{util::*, FFInput};
use ffmpeg::ffi::AVFormatContext;
use ffmpeg::ffi::{avformat_seek_file, AVSEEK_FLAG_BACKWARD};
use ffmpeg::frame::Audio;
use ffmpeg::frame::Video;
use ffmpeg_next as ffmpeg;

pub fn read_video_raw(
    ctx: &mut FFInput,
    index: usize,
    time: f64,
    vec_buf: &mut Vec<Video>,
    //exact_data: bool,
) -> Result<Vec<Ordering>> {
    let buf_index_tmp: Vec<Ordering> = Vec::new();
    let input = ctx.ctx.get_mut().unwrap();
    let video = match ctx
        .children
        .get_mut(index)
        .ok_or_else(|| anyhow!("index is not valid"))?
    {
        FFInputChild::Video(v) => v,
        FFInputChild::Audio(_) => return Err(anyhow!("audio index in read_video")),
    };

    let avg_frame_rate = video.afr.0 as f64 / video.afr.1 as f64;
    let pts = time2ts(time, (video.tb.0, video.tb.1));

    //println!("vid tb: {:?}, pts: {:?}", video.tb, pts);

    // println!(
    //     "pts: {} , pts->f_num: {}",
    //     pts,
    //     ts2frame_num(pts, (video.tb.0, video.tb.1), avg_frame_rate)
    // );

    //if requested timestamp <= last timestamp that read
    if video.last_pts >= pts {
        println!("seek");
        video.decoder.flush();
        unsafe {
            seek(input.as_mut_ptr(), video.index as i32, pts)?;
        }
    }
    println!("pts: {pts}");

    let pkts = input.packets();

    for (i, v) in vec_buf.iter().enumerate() {
        match v.pts().unwrap().cmp(&pts) {
            Ordering::Less => buf_index_tmp.push(Ordering::Less),
            Ordering::Equal => buf_index_tmp.push(Ordering::Equal),
            Ordering::Greater => {
                if i == 0 {
                    panic!("zero vec greater, seek required")
                } else {
                    buf_index_tmp.push(Ordering::Equal)
                }
            }
        }
    }

    for i in pkts {
        if i.0.index() == video.index {
            loop {
                let mut v = Video::empty();
                let ord = read_raw_frame(&mut v, Some(&i.1), &mut video.decoder, false, &pts)?;
                match ord {
                    Some(s) => {
                        vec_buf.push(v);
                        buf_index_tmp.push(s);
                    }
                    None => {
                        if buf_index_tmp.last() != None
                            && buf_index_tmp.last() != Some(&Ordering::Less)
                        {
                            return Ok(buf_index_tmp);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    loop {
        let mut v = Video::empty();
        let ord = read_raw_frame(&mut v, None, &mut video.decoder, false, &pts)?;
        match ord {
            Some(s) => {
                vec_buf.push(v);
                buf_index_tmp.push(s);
            }
            None => {
                if buf_index_tmp.last() != None && buf_index_tmp.last() != Some(&Ordering::Less) {
                    return Ok(buf_index_tmp);
                } else {
                    break;
                }
            }
        }
    }

    video.decoder.flush();
    panic!("irregular");
}

pub fn read_audio_raw(
    ctx: &mut FFInput,
    index: usize,
    time: f64,
    vec_buf: &mut Vec<Audio>,
) -> Result<Vec<Ordering>> {
    //TODO: Audio refactor like read_video_raw
    // println!("audio");
    let mut audio_vec = Vec::<Audio>::new();
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

    //println!("aud tb: {:?}, pts: {:?}", audio.tb, pts);

    //if requested timestamp <= last timestamp that read
    if audio.last_pts >= pts {
        println!("seek");
        audio.decoder.flush();
        unsafe {
            seek(input.as_mut_ptr(), audio.index as i32, pts)?;
        }
    }

    //println!("audio_tb : {}", audio.tb);

    let mut b = false;
    loop {
        let ord = read_raw_frame(&mut a, None, &mut audio.decoder, true, &pts)?;
        match ord {
            Some(s) => match s {
                Ordering::Less => {
                    if !exact_data {
                        b = true;
                        audio_vec.push(a);
                        a = Audio::empty();
                    }
                }
                Ordering::Equal => {
                    b = true;
                    break;
                }
                Ordering::Greater => break,
            },
            None => break,
        }
    }
    println!("first_loop_end");

    //let mut b = read_raw_frame(&mut a, None, &mut audio.decoder, true, &pts)?;

    if !b {
        'outer: for (s, p) in input.packets() {
            if s.index() == audio.index {
                loop {
                    let ord = read_raw_frame(&mut a, Some(&p), &mut audio.decoder, false, &pts)?;
                    match ord {
                        Some(s) => match s {
                            Ordering::Less => {
                                if !exact_data {
                                    b = true;
                                    audio_vec.push(a);
                                    a = Audio::empty();
                                }
                            }
                            Ordering::Equal => {
                                b = true;
                                break 'outer;
                            }
                            Ordering::Greater => break 'outer,
                        },
                        None => break 'outer,
                    }
                }
            }
        }
    }

    if !b {
        loop {
            let ord = read_raw_frame(&mut a, None, &mut audio.decoder, false, &pts)?;
            match ord {
                Some(s) => match s {
                    Ordering::Less => {
                        if !exact_data {
                            b = true;
                            audio_vec.push(a);
                            a = Audio::empty();
                        }
                    }
                    Ordering::Equal => {
                        b = true;
                        break;
                    }
                    Ordering::Greater => break,
                },
                None => break,
            }
        }
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
    ts: &i64,
) -> Result<Option<Ordering>> {
    if !read_only {
        match packet {
            Some(s) => decoder.send_packet(s)?,
            None => decoder.send_eof()?,
        };
    }

    if decoder.receive_frame(frame).is_ok() {
        println!("read : {}", frame.pts().unwrap());
        return Ok(Some(frame.pts().unwrap().cmp(ts)));
    }
    Ok(None)
}

pub fn get_fps(ctx: &mut FFInput, index: usize) -> NRRational {
    match &ctx.children[index] {
        FFInputChild::Video(v) => v.afr.into(),
        FFInputChild::Audio(_) => todo!(),
    }
}
