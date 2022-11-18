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
use super::util::*;
use super::*;
use ffmpeg::ffi::AVFormatContext;

pub fn read_video(ctx: &mut FFInput, index: usize, time: f64, frame: &mut Frame) -> Result<bool> {
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
            seek(input.as_mut_ptr(), video.index as i32, pts)?;
        }
    }

    let pkts = input.packets();
    let mut v = Video::empty();
    let decoder = &mut video.decoder;
    let mut b = read_raw_frame(&mut v, None, decoder, true, pts)?;

    if !b {
        'outer: for i in pkts {
            match i.0.index().cmp(&video.index) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => {
                    b = read_raw_frame(&mut v, Some(&i.1), decoder, false, pts)?;
                    if b {
                        break 'outer;
                    } else {
                        panic!("found packets but can't accquire frame in video");
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

    //if requested timestamp <= last timestamp that read
    if audio.last_pts >= pts {
        println!("seek");
        audio.decoder.flush();
        unsafe {
            seek(input.as_mut_ptr(), audio.index as i32, pts)?;
        }
    }

    println!("audio_tb : {}", audio.tb);

    let mut b = read_raw_frame(&mut a, None, &mut audio.decoder, true, pts)?;

    if !b {
        'outer: for (s, p) in input.packets() {
            if s.index() == audio.index {
                match s.index().cmp(&audio.index) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => {
                        b = read_raw_frame(&mut a, Some(&p), &mut audio.decoder, false, pts)?;
                        if b {
                            break 'outer;
                        } else {
                            panic!("found packets but can't accquire frame in audio");
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
