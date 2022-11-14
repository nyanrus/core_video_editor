use std::sync::Mutex;

use anyhow::Result;
use ffmpeg::ffi::{avformat_seek_file, AVSEEK_FLAG_BACKWARD};
use ffmpeg::format::{self, Pixel};
use ffmpeg::media::Type;

use ffmpeg::software::scaling::flag::Flags;
use ffmpeg::util::frame::video::Video;
use ffmpeg_next as ffmpeg;
use rayon::prelude::*;

use crate::base::frame::Frame;

fn ts2time(pts: i64, time_base: (i32, i32)) -> f64 {
    pts as f64 * time_base.0 as f64 / time_base.1 as f64
}

fn time2ts(time: f64, time_base: (i32, i32)) -> i64 {
    (time * time_base.1 as f64 / time_base.0 as f64).round() as i64
}

fn frame_num2time(frame_num: u32, fps: f64) -> f64 {
    frame_num as f64 / fps
}

fn time2frame_num(time: f64, fps: f64) -> u32 {
    (time * fps).round() as u32
}

fn ts2frame_num(pts: i64, time_base: (i32, i32), fps: f64) -> u32 {
    time2frame_num(ts2time(pts, time_base), fps)
}

fn frame_num2ts(frame_num: u32, time_base: (i32, i32), fps: f64) -> i64 {
    time2ts(frame_num2time(frame_num, fps), time_base)
}

pub struct FFctxInput {
    pub ctx: Mutex<format::context::Input>,
    pub last_pts: i64,
    pub video_decoder: ffmpeg::decoder::Video,
    pub scaler: ffmpeg::software::scaling::Context,
}

pub fn init(path: &str) -> Result<FFctxInput> {
    match format::input(&path) {
        Ok(o) => {
            let a = o.streams().best(Type::Video).unwrap();
            let context_decoder = ffmpeg::codec::context::Context::from_parameters(a.parameters())?;
            let decoder = context_decoder.decoder().video()?;
            let scaler = ffmpeg::software::scaling::Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                Pixel::RGBA,
                decoder.width(),
                decoder.height(),
                Flags::BILINEAR,
            )?;
            Ok(FFctxInput {
                ctx: Mutex::new(o),
                last_pts: -1,
                video_decoder: decoder,
                scaler,
            })
        }
        Err(_) => todo!(),
    }
}

pub fn read(
    //o: &mut ffmpeg::format::context::Input,
    ctx: &mut FFctxInput,
    num: usize,
) -> Result<crate::base::frame::Frame> {
    //o.seek(num, ..).unwrap();
    let o = ctx.ctx.get_mut().unwrap();

    let a = o.streams().best(Type::Video).unwrap();

    let video_stream_index = a.index();

    let tb = a.time_base();

    let afr = a.avg_frame_rate();
    let fr = afr.0 as f64 / afr.1 as f64;
    println!("{}", num);

    let pts = frame_num2ts(num as u32, (tb.0, tb.1), fr);
    println!(
        "pts: {} , pts->f_num: {}",
        pts,
        ts2frame_num(pts, (tb.0, tb.1), fr)
    );

    // o.seek(ts, i64::min_value()..ts).unwrap();

    if ctx.last_pts >= pts {
        println!("seek");
        unsafe {
            if 0 > avformat_seek_file(
                o.as_mut_ptr(),
                video_stream_index as i32,
                i64::min_value(),
                pts,
                pts,
                AVSEEK_FLAG_BACKWARD,
            ) {
                panic!("")
            }
        }
    }

    let pkts = o.packets();

    let mut b = false;

    let mut foobool = true;

    let decoder = &mut ctx.video_decoder;

    let mut v = Video::empty();

    while decoder.receive_frame(&mut v).is_ok() {
        //println!("yes ok!");
        if foobool {
            foobool = false;
            println!(
                "key_ts, {} , key_ts->f_num, {}",
                v.pts().unwrap(),
                ts2frame_num(v.pts().unwrap(), (tb.0, tb.1), fr)
            );
        }
        //println!("vpts {}", v.pts().unwrap());
        if v.pts().unwrap() == pts {
            //println!("b is true!");
            ctx.last_pts = pts;
            b = true;
            break;
        }
    }
    if !b {
        for i in pkts {
            if i.0.index() == video_stream_index {
                decoder.send_packet(&i.1).unwrap();
                while decoder.receive_frame(&mut v).is_ok() {
                    //println!("yes ok!");
                    if foobool {
                        foobool = false;
                        println!(
                            "key_ts, {} , key_ts->f_num, {}",
                            v.pts().unwrap(),
                            ts2frame_num(v.pts().unwrap(), (tb.0, tb.1), fr)
                        );
                    }
                    //println!("vpts {}", v.pts().unwrap());
                    if v.pts().unwrap() == pts {
                        //println!("b is true!");
                        ctx.last_pts = pts;
                        b = true;
                        break;
                    }
                }
                if b {
                    break;
                }
            }
        }
    }

    if !b {
        decoder.send_eof().unwrap();
        while decoder.receive_frame(&mut v).is_ok() {
            if v.pts().unwrap() == pts {
                b = true;
                break;
            }
        }
    }

    if !b {
        decoder.flush();
        panic!("irregular")
    }

    let mut fff = Frame::init(decoder.width() as usize, decoder.height() as usize);

    let mut vvv = Video::empty();

    let scaler = &mut ctx.scaler;

    scaler.run(&v, &mut vvv).unwrap();

    vvv.data(0)
        .par_chunks(4)
        .map(|x| [x[0], x[1], x[2], x[3]])
        .collect_into_vec(&mut fff.vec_rgba);

    //println!("{}", fff.vec_rgba.len());

    println!("end");
    Ok(fff)
}
