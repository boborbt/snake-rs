use std::io::{ Read, Write };
use std::thread;
use std::time::Duration;

use termion::{ AsyncReader, terminal_size, cursor, clear };

use crate::renderable::{
    Renderable,
    CenteredPanel,
    MAIN_MENU_SCREEN
};

use crate::scores::ScoreBoard;

pub(crate) enum MainMenuChoice {
    EasyMode,
    HardMode,
    Quit
}

pub(crate) fn run<W:Write>(stdin:&mut AsyncReader, stdout:&mut W, score_board: ScoreBoard) -> MainMenuChoice {
    let panel = CenteredPanel {
        content: MAIN_MENU_SCREEN.to_vec(),
        field: terminal_size().unwrap()
    };
    write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
    score_board.render(stdout);

    panel.render(stdout);
    stdout.flush().unwrap();

    loop {
        let mut buf = [0; 1];
        stdin.read(&mut buf).unwrap();

        match buf[0] {
            b'1' => {
                return MainMenuChoice::EasyMode;
            },
            b'2' => {
                return MainMenuChoice::HardMode;
            },
            b'q' => {
                return MainMenuChoice::Quit;
            },
            _ => ()
        }

        thread::sleep(Duration::from_millis(100));
    }
}

