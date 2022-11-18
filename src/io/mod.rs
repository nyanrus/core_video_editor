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

use crate::base::interface::ProcessInterface;

pub mod input {
    use super::ProcessInterface;
    pub trait InputInterface<TData, TSettings> {
        fn in_open_file(&self, file: &str) -> Option<Box<dyn ProcessInterface<TData, TSettings>>>;
    }
}
pub mod output {
    use super::ProcessInterface;
    pub trait OutputInterface<TData, TSettings> {
        fn out_open_file(&self, file: &str) -> Option<Box<dyn ProcessInterface<TData, TSettings>>>;
    }
}

pub mod filter {
    use super::ProcessInterface;

    pub trait FilterInterface<TData, TSettings> {
        fn get_fi(&self) -> Box<dyn ProcessInterface<TData, TSettings>>;
    }
}
