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

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Difficulty {
    Easy,
    Hard
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum MenuAction {
    StartGame(Difficulty, Option<(u16, u16)>),
    Quit
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => "Easy".to_string(),
            Difficulty::Hard => "Hard".to_string()
        }
    }
}

impl ToString for MenuAction {
    fn to_string(&self) -> String {
        match self {
            MenuAction::StartGame(difficulty, size) => {
                let size = match size {
                    Some((w,h)) => format!("{}x{}", w, h),
                    None => "Full".to_string()
                };
                format!("Start {} mode ({})", difficulty.to_string(), size)
            },
            MenuAction::Quit => "Quit".to_string()
        }
    }
}


pub(crate) fn run<W:Write>(stdin:&mut AsyncReader, stdout:&mut W, score_board: ScoreBoard) -> MenuAction {
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
                return MenuAction::StartGame(Difficulty::Easy, None);
            },
            b'2' => {
                return MenuAction::StartGame(Difficulty::Hard, None)
            },
            b'3' => {
                return MenuAction::StartGame(Difficulty::Easy, Some((80,25)));
            },
            b'4' => {
                return MenuAction::StartGame(Difficulty::Hard, Some((80,25)));
            },
            b'q' => {
                return MenuAction::Quit;
            },
            _ => ()
        }
    }
}

