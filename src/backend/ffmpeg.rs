use anyhow::Result;
use ffmpeg::ffi::avformat_seek_file;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;

use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg::{Packet, Stream};
use ffmpeg_next as ffmpeg;
use opencv::highgui::set_opengl_context;
use rayon::prelude::*;

use crate::base::frame::Frame;

pub fn read(path: &str, num: usize) -> Result<crate::base::frame::Frame> {
    match ffmpeg::format::input(&path) {
        Ok(mut o) => {
            //o.seek(num, ..).unwrap();
            let a = o.streams().best(Type::Video).unwrap();
            let mut v = Video::empty();
            let context_decoder = ffmpeg::codec::context::Context::from_parameters(a.parameters())?;
            let mut decoder = context_decoder.decoder().video()?;
            let video_stream_index = a.index();

            //println!("{:?}", decoder.format());

            let mut scaler = ffmpeg::software::scaling::Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                Pixel::RGBA,
                decoder.width(),
                decoder.height(),
                Flags::BILINEAR,
            )?;

            let mut f_rgb = Video::empty();

            let nb_st = o.nb_streams();

            let bar = o
                .packets()
                .enumerate()
                .map(|(_, x)| x)
                .collect::<Vec<(Stream, Packet)>>();

            let mut left_i = num;

            //let mut i = num * nb_st as usize;

            let mut num_i = 0;
            let mut ii = 0;

            loop {
                if bar[num_i].0.index() == video_stream_index {
                    if ii == num {
                        break;
                    }
                    ii += 1;
                }
                num_i += 1;
            }

            let mut i = num_i;

            //println!("i : {}", i);

            loop {
                if bar[i].0.index() == video_stream_index {
                    // println!(
                    //     "idx {} key {} {} {}",
                    //     bar[i].0.index(),
                    //     bar[i].1.is_key(),
                    //     left_i,
                    //     i,
                    //     //bar[i].1
                    // );
                    if bar[i].1.is_key() {
                        break;
                    }
                    left_i -= 1;
                }
                if i == 0 {
                    panic!("59 panic");
                }
                i -= 1;
            }

            let foo = bar
                .iter()
                .enumerate()
                .filter(|(i, x)| (left_i) * nb_st as usize <= *i)
                .map(|(_, x)| x)
                .collect::<Vec<&(Stream, Packet)>>();

            //println!("{} {}", foo.len(), num * nb_st as usize);

            //println!("{} {}", num, left_i);

            let f_idx = left_i;
            let mut ff_idx = left_i;

            //println!("{}", f_idx);
            let i = foo.iter();

            let mut b = false;

            for (stream, packet) in i {
                if stream.index() == video_stream_index {
                    //println!("{}", packet.is_key());
                    //println!("{}", packet.is_corrupt());

                    decoder.send_packet(packet).unwrap();
                    while decoder.receive_frame(&mut v).is_ok() {
                        if ff_idx != num {
                            ff_idx += 1;
                            //println!("{}", ff_idx);
                            continue;
                        }
                        scaler.run(&v, &mut f_rgb)?;
                        ff_idx += 1;
                        b = true;
                        break;
                    }

                    if b {
                        break;
                    }
                    //println!("send");
                }
            }
            decoder.send_eof().unwrap();

            loop {
                if b {
                    break;
                }
                while decoder.receive_frame(&mut v).is_ok() {
                    if ff_idx != num {
                        ff_idx += 1;
                        //println!("{}", ff_idx);
                        continue;
                    }
                    scaler.run(&v, &mut f_rgb)?;
                    //decoder.send_eof().unwrap();
                    ff_idx += 1;

                    break;
                }

                if ff_idx > num {
                    break;
                }
            }
            decoder.flush();

            let mut fff = Frame::init(decoder.width() as usize, decoder.height() as usize);

            println!("wow");

            f_rgb
                .data(0)
                .par_chunks(4)
                .map(|x| [x[0], x[1], x[2], x[3]])
                .collect_into_vec(&mut fff.vec_rgba);

            Ok(fff)
        }
        Err(e) => todo!(),
    }
}
