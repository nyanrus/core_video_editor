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
    pub interface: Arc<dyn ProcessInterface<TData, TSettings> + Sync>,
    pub cache_data: Mutex<LruCache<usize, Arc<Mutex<TData>>>>,
    pub ulid: Ulid,
}

unsafe impl<TData, TSettings> Send for Cached<TData, TSettings> {}

impl ProcessInterface<Frame, FrameSettings> for Cached<Frame, FrameSettings> {
    fn get_json_template(&self) -> anyhow::Result<serde_json::Value> {
        self.interface.get_json_template()
    }

    fn get_ulid(&self) -> Ulid {
        self.ulid
    }

    fn process(
        &self,
        f: &mut Frame,
        settings: &FrameSettings,
        json: serde_json::Value,
    ) -> anyhow::Result<bool> {
        let cache_data_ptr = &self.cache_data;
        let mut cache_data = match cache_data_ptr.try_lock() {
            Some(s) => s,
            None => return Err(anyhow::anyhow!("cache process lock err")),
        };
        {
            let interface = self.interface.clone();

            if !cache_data.contains(&(settings.frame_num)) {
                let mut frame = Box::new(Frame::init(f.w, f.h));
                interface.process(&mut frame, settings, json.clone());
                cache_data.push(settings.frame_num, Arc::new(Mutex::new(*frame)));
            }

            *f = cache_data
                .pop(&(settings.frame_num))
                .unwrap()
                .lock()
                .clone();
        }

        let settings = settings.clone();
        let frame_num = settings.frame_num;
        let cache_size = self.cache_size;
        let (w, h) = (f.w, f.h);
        let mut cache_data_vec = Vec::new();
        for i in 1..cache_size {
            let contains = cache_data.contains(&(frame_num + i));
            if !contains {
                cache_data.push(frame_num + i, Arc::new(Mutex::new(Frame::init(w, h))));
                let cache_data = cache_data.get(&(frame_num + i)).unwrap().clone();
                cache_data_vec.push(cache_data);
            }
        }
        for i in cache_data_vec {
            let interface = self.interface.clone();
            let settings = settings.clone();
            let json = json.clone();
            let ptr = i.clone();
            std::thread::spawn(move || {
                let mut frame = Box::new(Frame::init(w, h));
                interface.process(&mut frame, &settings, json);

                *ptr.lock() = *frame;
            });
        }

        Ok(true)
    }
}

impl ProcessInterface<Vec<f32>, FrameSettings> for Cached<Vec<f32>, FrameSettings> {
    fn get_json_template(&self) -> anyhow::Result<serde_json::Value> {
        self.interface.get_json_template()
    }

    fn get_ulid(&self) -> Ulid {
        self.ulid
    }

    fn process(
        &self,
        f: &mut Vec<f32>,
        settings: &FrameSettings,
        json: serde_json::Value,
    ) -> anyhow::Result<bool> {
        //self.cache_data.contains(k)
        let cache_data = &mut self
            .cache_data
            .try_lock()
            .ok_or_else(|| anyhow::anyhow!("cache process lock err"))?;
        let interface = self.interface.clone();
        {
            if !cache_data.contains(&(settings.frame_num)) {
                let mut frame = Vec::new();
                interface.process(&mut frame, settings, json.clone());
                cache_data.push(settings.frame_num, Arc::new(Mutex::new(frame)));
            }

            *f = cache_data
                .pop(&(settings.frame_num))
                .unwrap()
                .lock()
                .clone();
        }

        let settings = settings.clone();
        let cache_size = self.cache_size;

        let frame_num = settings.frame_num;

        for i in 1..cache_size {
            if !cache_data.contains(&(frame_num + i)) {
                cache_data.push(frame_num + i, Arc::new(Mutex::new(Vec::new())));
                let cache_data = cache_data.get(&(frame_num + i)).unwrap().clone();

                let interface = self.interface.clone();
                let settings = settings.clone();
                let json = json.clone();
                std::thread::spawn(move || {
                    let mut frame = Vec::new();
                    interface.process(&mut frame, &settings, json);

                    *cache_data.lock() = frame;
                });
            }
        }

        Ok(true)
    }
}
