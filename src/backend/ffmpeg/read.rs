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
        panic!("no data in video")
    }
}

impl FFAudio {
    pub fn read_audio(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
        samples: &mut Vec<f32>,
    ) -> Result<()> {
        let audio_vec = self.read_audio_raw_wrap(input, time)?;
        samples.clear();
        for i in audio_vec {
            let mut a = Audio::empty();
            self.resampler.run(&i, &mut a)?;
            samples.append(&mut bytemuck::cast_slice(a.data(0)).to_vec());
        }
        Ok(())
    }

    fn read_audio_raw_wrap(
        &mut self,
        input: &mut format::context::Input,
        time: f64,
    ) -> Result<Vec<Audio>> {
        // let mut last_pts = -1i64;
        let mut vec = Vec::new();
        for (i, o) in read_audio_raw(self, input, time)?.iter() {
            match o {
                Ordering::Less => {
                    vec.push(self.cache_buf.remove(i).unwrap());
                    // if let Some(s) = self.cache_buf.remove(&last_pts) {
                    //     vec.push(s)
                    // }
                    // last_pts = *i;
                }
                Ordering::Equal => return Ok(vec),
                Ordering::Greater => return Ok(vec),
            }
        }
        panic!("no data in audio")
    }
}
