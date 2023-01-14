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

use crate::base::frame::FrameSettings;
use crate::base::{frame::Frame, interface::ProcessInterface};

use crate::io::input::InputInterface;

use self::base::FFInput;

pub mod base;
pub mod read;
pub mod read_raw;
pub mod util;

pub struct FFInit();

impl InputInterface<Frame, FrameSettings> for FFInit {
    fn in_open_file(&self, _file: &str) -> Option<Box<dyn ProcessInterface<Frame, FrameSettings>>> {
        match FFInput::init(_file) {
            Ok(o) => Some(Box::new(o) as Box<dyn ProcessInterface<Frame, FrameSettings>>),
            Err(_e) => None,
        }
    }
}

impl InputInterface<Vec<f32>, FrameSettings> for FFInit {
    fn in_open_file(
        &self,
        _file: &str,
    ) -> Option<Box<dyn ProcessInterface<Vec<f32>, FrameSettings>>> {
        match FFInput::init(_file) {
            Ok(o) => Some(Box::new(o) as Box<dyn ProcessInterface<Vec<f32>, FrameSettings>>),
            Err(_e) => None,
        }
    }
}

// pub struct FFInitCached();
// impl InputInterface<Frame, FrameSettings> for FFInitCached {
//     fn in_open_file(&self, file: &str) -> Option<Box<dyn ProcessInterface<Frame, FrameSettings>>> {
//         match FFInit().in_open_file(file) {
//             Some(s) => Some(Box::new(Cached::<Frame, FrameSettings> {
//                 cache_size: 3,
//                 interface: Arc::from(s),
//                 cache_data: Mutex::new(LruCache::new(NonZeroUsize::new(3).unwrap())),
//                 ulid: Ulid::new(),
//             })),
//             None => None,
//         }
//     }
// }

// impl InputInterface<Vec<f32>, FrameSettings> for FFInitCached {
//     fn in_open_file(
//         &self,
//         file: &str,
//     ) -> Option<Box<dyn ProcessInterface<Vec<f32>, FrameSettings>>> {
//         match FFInit().in_open_file(file) {
//             Some(s) => Some(Box::new(Cached::<Vec<f32>, FrameSettings> {
//                 cache_size: 10,
//                 interface: Arc::from(s),
//                 cache_data: Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())),
//                 ulid: Ulid::new(),
//             })),
//             None => None,
//         }
//     }
// }
