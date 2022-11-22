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

use std::sync::Arc;

use lru::LruCache;
use parking_lot::Mutex;
use ulid::Ulid;

use crate::base::{
    frame::{Frame, FrameSettings},
    interface::ProcessInterface,
};

pub struct Cached<TData, TSettings> {
    pub cache_size: usize,
    pub interface: Arc<Mutex<Box<dyn ProcessInterface<TData, TSettings>>>>,
    pub cache_data: Arc<Mutex<LruCache<usize, Box<TData>>>>,
    pub ulid: Ulid,
}

impl ProcessInterface<Frame, FrameSettings> for Cached<Frame, FrameSettings> {
    fn get_json_template(&self) -> serde_json::Value {
        self.interface.lock().get_json_template()
    }

    fn get_ulid(&self) -> Ulid {
        self.ulid
    }

    fn process(
        &mut self,
        f: &mut Box<Frame>,
        settings: &FrameSettings,
        json: serde_json::Value,
    ) -> bool {
        //self.cache_data.contains(k)
        //println!("first");
        let cache_data = self.cache_data.clone();
        let interface = self.interface.clone();
        {
            let mut cache_data = cache_data.lock();
            let mut interface = interface.lock();
            //println!("lock");
            if !cache_data.contains(&(settings.frame_num)) {
                let mut frame = Box::new(Frame::init(f.w, f.h));
                interface.process(&mut frame, settings, json.clone());
                cache_data.push(settings.frame_num, frame);
            }

            *f = cache_data.pop(&(settings.frame_num)).unwrap();
        }
        //println!("done");

        let settings = settings.clone();
        let cache_size = self.cache_size;
        let (w, h) = (f.w, f.h);
        std::thread::spawn(move || {
            let mut cache_data = cache_data.lock();
            let mut interface = interface.lock();
            for i in 1..cache_size {
                //println!("i: {}", i);
                if !cache_data.contains(&(settings.frame_num + i)) {
                    let mut frame = Box::new(Frame::init(w, h));
                    interface.process(&mut frame, &settings, json.clone());
                    cache_data.push(settings.frame_num + i, frame);
                }
            }
        });

        true
    }
}

impl ProcessInterface<Vec<f32>, FrameSettings> for Cached<Vec<f32>, FrameSettings> {
    fn get_json_template(&self) -> serde_json::Value {
        self.interface.lock().get_json_template()
    }

    fn get_ulid(&self) -> Ulid {
        self.ulid
    }

    fn process(
        &mut self,
        f: &mut Box<Vec<f32>>,
        settings: &FrameSettings,
        json: serde_json::Value,
    ) -> bool {
        //self.cache_data.contains(k)
        let cache_data = self.cache_data.clone();
        let interface = self.interface.clone();
        {
            let mut cache_data = cache_data.lock();
            let mut interface = interface.lock();
            //println!("lock");
            if !cache_data.contains(&(settings.frame_num)) {
                let mut frame = Box::new(Vec::new());
                interface.process(&mut frame, settings, json.clone());
                cache_data.push(settings.frame_num, frame);
            }

            *f = cache_data.pop(&(settings.frame_num)).unwrap();
        }
        //println!("done");

        let settings = settings.clone();
        let cache_size = self.cache_size;
        std::thread::spawn(move || {
            let mut cache_data = cache_data.lock();
            let mut interface = interface.lock();
            for i in 1..cache_size {
                //println!("i: {}", i);
                if !cache_data.contains(&(settings.frame_num + i)) {
                    let mut frame = Box::new(Vec::new());
                    interface.process(&mut frame, &settings, json.clone());
                    cache_data.push(settings.frame_num + i, frame);
                }
            }
        });

        true
    }
}
