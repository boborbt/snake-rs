// Snake implementation in Rust
// Copyright (C) 2023  Roberto Esposito

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

mod gameobjs;
mod renderable;
mod app;

use termion::{
    raw::IntoRawMode,
    async_stdin
};

use std::{
    env,
    io::{ stdout },
};

use crate::app::App;


fn main() {
    let args: Vec<String> = env::args().collect();
    let easy_mode = args.len() > 1 && args[1] == "--easy";
        

    let stdout = stdout();
    let mut stdin = async_stdin();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    stdout.activate_raw_mode().unwrap();
    
    App::run(&mut stdin, &mut stdout, easy_mode);
}
