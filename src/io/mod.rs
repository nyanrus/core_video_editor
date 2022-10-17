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

use crate::base::frame::FrameInterface;

pub mod input {
    use super::FrameInterface;
    pub trait InputInterface<T> {
        fn in_open_file(&self, file: &str) -> Option<Box<dyn FrameInterface<T>>>;
    }
}
pub mod output {
    use super::FrameInterface;
    pub trait OutputInterface<T> {
        fn out_open_file(&self, file: &str) -> Option<Box<dyn FrameInterface<T>>>;
    }
}

pub mod filter {
    use super::FrameInterface;

    pub trait FilterInterface<T> {
        fn get_fi(&self) -> Box<dyn FrameInterface<T>>;
    }
}
