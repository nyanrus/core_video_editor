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

use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use ffmpeg_next::{
    format,
    frame::{Audio, Video},
};

use super::{
    base::{FFAudio, FFVideo},
    read_raw::{read_audio_raw, read_video_raw},
    util::time2ts,
};
use crate::base::frame::Frame;

impl FFVideo {
    pub fn read_video(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
        frame: &mut Frame,
    ) -> Result<()> {
        let mut v = Video::empty();
        frame.vec_rgba.clear();
        let tmp = self.read_video_raw_wrap(input, time)?.clone();
        if tmp.data(0).is_empty() {
            return Ok(());
        }
        self.scaler.run(&tmp, &mut v)?;
        frame.vec_rgba = v.data(0).to_vec();
        Ok(())
    }

    fn read_video_raw_wrap(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
    ) -> Result<&Video> {
        let mut last_pts = -1i64;
        for (i, o) in read_video_raw(self, input, time)?.iter() {
            match o {
                Ordering::Less => {
                    self.cache_buf.remove(&last_pts);
                    last_pts = *i;
                }
                Ordering::Equal => return Ok(&self.cache_buf[i]),
                Ordering::Greater => {
                    println!("last_pts : {}", last_pts);
                    return self
                        .cache_buf
                        .get(&last_pts)
                        .ok_or_else(|| anyhow!("Seek Required"));
                }
            }
        }
        self.cache_buf.insert(-2, Video::empty());
        Ok(self.cache_buf.get(&-2).unwrap())
        //panic!("no data in video")
    }
}

impl FFAudio {
    pub fn read_audio(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
        samples: &mut Vec<f32>,
        length: f64,
    ) -> Result<()> {
        let audio_vec = self.read_audio_raw_wrap(input, time, length)?;
        //samples.clear();
        let pts = time2ts(time, self.time_base);
        let pts2 = time2ts(time + length, self.time_base);
        let mut data = Vec::<f32>::new();

        let (first_pts, last_pts) = (
            audio_vec.first().unwrap().pts().unwrap(),
            audio_vec.last().unwrap().pts().unwrap() + audio_vec.last().unwrap().samples() as i64
                - 1,
        );

        for i in audio_vec {
            let mut a = Audio::empty();
            loop {
                let result = self.resampler.run(&i, &mut a)?;
                let mut vec = (bytemuck::cast_slice(a.data(0)) as &[f32]).to_vec();
                data.append(&mut vec);

                match result {
                    Some(_s) => self.resampler.flush(&mut a)?,
                    None => break,
                };
            }
        }

        let (first_diff, last_diff) = (pts - first_pts, last_pts - pts2);

        //println!("pts: {} {}", first_pts, last_pts);

        let mut data = data[(first_diff * self.decoder.channels() as i64) as usize
            ..data.len() - (last_diff * self.decoder.channels() as i64) as usize]
            .to_vec();
        //println!("{}", data.len());
        samples.append(&mut data);
        Ok(())
    }

    fn read_audio_raw_wrap(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
        length: f64,
    ) -> Result<Vec<Audio>> {
        //println!("time len : {} {}", time, length);
        let mut last_pts = -1i64;
        let mut vec = Vec::<Audio>::new();
        let pts = time2ts(time, self.time_base);
        for (i, o) in read_audio_raw(self, input, time + length)?.iter() {
            match o {
                Ordering::Less => {
                    if pts >= *i {
                        self.cache_buf.remove(&last_pts);
                        last_pts = *i;
                    } else {
                        if let Some(s) = self.cache_buf.get(&last_pts) {
                            vec.push(s.clone());
                        }

                        last_pts = *i;
                    }
                }
                Ordering::Equal => {
                    vec.push(self.cache_buf.get(&last_pts).unwrap().clone());
                    return Ok(vec);
                }
                Ordering::Greater => {
                    if let Some(s) = self.cache_buf.get(&last_pts) {
                        vec.push(s.clone());
                    }

                    vec.push(self.cache_buf.get(i).unwrap().clone());
                    return Ok(vec);
                    //println!("last_pts : {}", last_pts);
                    // return self
                    //     .cache_buf
                    //     .get(&last_pts)
                    //     .ok_or_else(|| anyhow!("Seek Required"));
                }
            }
        }
        Ok(vec)
        //panic!("no data in audio")
    }
}
