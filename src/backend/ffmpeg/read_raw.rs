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

use super::{
    base::{FFAudio, FFInputChild, FFVideo},
    util::*,
    FFInput,
};
use anyhow::Result;
use std::{cmp::Ordering, collections::BTreeMap};

use ffmpeg::ffi::{av_seek_frame, AVSEEK_FLAG_BACKWARD};
use ffmpeg::format;
use ffmpeg::frame::{Audio, Video};
use ffmpeg_next as ffmpeg;

//* before using, seek required!!
#[allow(unused_assignments)]
pub fn read_video_raw(
    ctx: &mut FFVideo,
    input: &mut format::context::Input,
    time: f64,
) -> Result<BTreeMap<i64, Ordering>> {
    let video = ctx;
    let cache_buf = &mut video.cache_buf;
    let mut buf_index_tmp = BTreeMap::new();

    let _avg_frame_rate = video.afr.0 as f64 / video.afr.1 as f64;
    let pts = time2ts(time, (video.time_base.0, video.time_base.1));

    //println!("pts: {}", pts);
    //println!("vid tb: {:?}, pts: {:?}", video.tb, pts);

    // println!(
    //     "pts: {} , pts->f_num: {}",
    //     pts,
    //     ts2frame_num(pts, (video.tb.0, video.tb.1), avg_frame_rate)
    // );

    if video.last_pts >= pts {
        video.decoder.flush();
        seek(input, video.index, pts).unwrap();
    }

    //TODO: 移植
    // //if requested timestamp <= last timestamp that read
    // if video.last_pts >= pts {
    //     println!("seek");
    //     video.decoder.flush();
    //     unsafe {
    //         seek(input.as_mut_ptr(), video.index as i32, pts)?;
    //     }
    // }
    // println!("pts: {pts}");
    //println!("first");

    let pkts = input.packets();

    for (i, v) in cache_buf.iter() {
        //println!("loop 1");
        //println!("index : {}", i);
        match v.pts().unwrap().cmp(&pts) {
            Ordering::Less => buf_index_tmp.insert(*i, Ordering::Less),
            Ordering::Equal => {
                buf_index_tmp.insert(*i, Ordering::Equal);
                return Ok(buf_index_tmp);
            }
            Ordering::Greater => {
                buf_index_tmp.insert(*i, Ordering::Greater);
                return Ok(buf_index_tmp);
            }
        };
    }

    for (s, p) in pkts {
        if s.index() == video.index {
            // println!("loop 2");
            let mut v = Video::empty();
            let mut ord = read_raw_frame(&mut v, Some(&p), &mut video.decoder, false, &pts)?;

            //println!("p,pts : {:?}", p.pts());
            //println!("ord: {:?}", ord);
            //println!("idx key : {}", v.pts().unwrap());
            let mut is_ge = false;
            loop {
                match ord {
                    Some(s) => {
                        let key = v.pts().unwrap();
                        //println!("key : {}", key);
                        //println!("ord: {:?}", s);
                        if s.is_ge() {
                            is_ge = true;
                        }
                        cache_buf.insert(key, v);
                        buf_index_tmp.insert(key, s);
                    }
                    None => {
                        if is_ge {
                            return Ok(buf_index_tmp);
                        } else {
                            break;
                        }
                    }
                }
                v = Video::empty();
                ord = read_raw_frame(&mut v, None, &mut video.decoder, true, &pts)?;
            }
        }
    }

    loop {
        //println!("loop 3");
        let mut v = Video::empty();
        let ord = read_raw_frame(&mut v, None, &mut video.decoder, false, &pts)?;

        let mut is_ge = false;
        match ord {
            Some(s) => {
                let key = v.pts().unwrap();
                if s.is_ge() {
                    is_ge = true;
                }
                cache_buf.insert(key, v);
                buf_index_tmp.insert(key, s);
            }
            None => {
                if is_ge {
                    return Ok(buf_index_tmp);
                } else {
                    break;
                }
            }
        }
    }

    video.decoder.flush();
    Ok(buf_index_tmp)
    //panic!("irregular");
}

//* before using, seek required!!
#[allow(unused_assignments)]
pub fn read_audio_raw(
    ctx: &mut FFAudio,
    input: &mut format::context::Input,
    time: f64,
) -> Result<BTreeMap<i64, Ordering>> {
    let audio = ctx;
    let cache_buf = &mut audio.cache_buf;
    let mut buf_index_tmp = BTreeMap::new();

    let pts = time2ts(time, audio.time_base);

    if audio.last_pts >= pts {
        audio.decoder.flush();
        seek(input, audio.index, pts).unwrap();
    }

    for (i, v) in cache_buf.iter() {
        match v.pts().unwrap().cmp(&pts) {
            Ordering::Less => buf_index_tmp.insert(*i, Ordering::Less),
            Ordering::Equal => {
                buf_index_tmp.insert(*i, Ordering::Equal);
                return Ok(buf_index_tmp);
            }
            Ordering::Greater => {
                buf_index_tmp.insert(*i, Ordering::Greater);
                return Ok(buf_index_tmp);
            }
        };
    }

    let pkts = input.packets();

    for (s, p) in pkts {
        if s.index() == audio.index {
            let mut a = Audio::empty();
            let mut ord = read_raw_frame(&mut a, Some(&p), &mut audio.decoder, false, &pts)?;
            let mut is_ge = false;
            loop {
                match ord {
                    Some(s) => {
                        let key = a.pts().unwrap();
                        if s.is_ge() {
                            is_ge = true;
                        }
                        cache_buf.insert(key, a);
                        buf_index_tmp.insert(key, s);
                    }
                    None => {
                        if is_ge {
                            return Ok(buf_index_tmp);
                        } else {
                            break;
                        }
                    }
                }
                a = Audio::empty();
                ord = read_raw_frame(&mut a, None, &mut audio.decoder, true, &pts)?;
            }
        }
    }

    loop {
        let mut a = Audio::empty();
        let ord = read_raw_frame(&mut a, None, &mut audio.decoder, false, &pts)?;
        let mut is_ge = false;
        match ord {
            Some(s) => {
                let key = a.pts().unwrap();
                if s.is_ge() {
                    is_ge = true;
                }
                cache_buf.insert(key, a);
                buf_index_tmp.insert(key, s);
            }
            None => {
                if is_ge {
                    return Ok(buf_index_tmp);
                } else {
                    break;
                }
            }
        }
    }

    audio.decoder.flush();
    Ok(buf_index_tmp)
    //panic!("irregular");
}

pub fn seek(input: &mut format::context::Input, idx: usize, ts: i64) -> Result<()> {
    unsafe {
        match av_seek_frame(input.as_mut_ptr(), idx as i32, ts, AVSEEK_FLAG_BACKWARD) {
            0.. => Ok(()),
            err => Err(anyhow::anyhow!("seek_error : {}", err)),
        }
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
        //println!("frame pts {}", frame.pts().unwrap());
        return Ok(Some(frame.pts().unwrap().cmp(ts)));
    }
    Ok(None)
}

pub fn get_fps(ctx: &mut FFInput, index: usize) -> NRRational {
    match &ctx.children.borrow()[index] {
        FFInputChild::Video(v) => v.afr.into(),
        FFInputChild::Audio(_) => todo!(),
    }
}
