use std::io::{ Write };
use serde::{ Serialize, Deserialize };

use termion::{ AsyncReader, terminal_size, cursor, clear };

use crate::renderable::{
    Frame,
    Renderable,
    CenteredPanel,
    MAIN_MENU_SCREEN
};

use crate::io::wait_char;

use crate::scores::ScoreBoard;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum MainMenuChoice {
    EasyMode,
    HardMode,
    EasyMode80x25,
    HardMode80x25,
    Quit
}

impl ToString for MainMenuChoice {
    fn to_string(&self) -> String {
        match self {
            MainMenuChoice::EasyMode => "Easy Mode".to_string(),
            MainMenuChoice::HardMode => "Hard Mode".to_string(),
            MainMenuChoice::EasyMode80x25 => "Easy Mode 80x25".to_string(),
            MainMenuChoice::HardMode80x25 => "Hard Mode 80x25".to_string(),
            MainMenuChoice::Quit => "Quit".to_string()
        }
    }
}


pub(crate) fn run<W:Write>(stdin:&mut AsyncReader, stdout:&mut W, score_board: ScoreBoard) -> MainMenuChoice {
    let panel = CenteredPanel {
        content: MAIN_MENU_SCREEN.to_vec(),
        frame: Frame::new((1,1), terminal_size().unwrap())
    };
    write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
    score_board.render(stdout);

    panel.render(stdout);
    stdout.flush().unwrap();

    loop {

        let char = wait_char(stdin);

        match char {
            b'1' => {
                return MainMenuChoice::EasyMode;
            },
            b'2' => {
                return MainMenuChoice::HardMode;
            },
            b'3' => {
                return MainMenuChoice::EasyMode80x25;
            },
            b'4' => {
                return MainMenuChoice::HardMode80x25;
            },
            b'q' => {
                return MainMenuChoice::Quit;
            },
            _ => ()
        }
    }
}

