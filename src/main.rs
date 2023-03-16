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
mod menu;
mod scores;
mod io;

use termion::{
    raw::IntoRawMode,
    async_stdin,
    clear,
    cursor
};

use std::{
    io::{ stdout, Write }
};

use crate::app::App;
use crate::menu::MainMenuChoice;
use crate::scores::{ScoreBoard, Difficulty};


fn main() {

    let stdout = stdout();
    let mut stdin = async_stdin();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut score_board = ScoreBoard::new();


    stdout.activate_raw_mode().unwrap();
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();    

    
    loop {
        let difficulty = match menu::run(&mut stdin, &mut stdout, score_board) {
                MainMenuChoice::Quit => None,
                MainMenuChoice::EasyMode => { Some(Difficulty::Easy) },
                MainMenuChoice::HardMode => { Some(Difficulty::Hard) },                    
            };
            
        if let Some(difficulty) = difficulty {
            let score = App::run(&mut stdin, &mut stdout, difficulty == Difficulty::Easy);
            score_board = score_board.update(score, difficulty);
        } else {
            break;
        }
    }
    write!(stdout, "{}", cursor::Show).unwrap();
}
