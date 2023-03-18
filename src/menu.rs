use std::io::{ Write };

use termion::{ AsyncReader, terminal_size, cursor, clear };

use crate::renderable::{
    Frame,
    Renderable,
    CenteredPanel,
    MAIN_MENU_SCREEN
};

use crate::io::wait_char;

use crate::scores::ScoreBoard;

pub(crate) enum MainMenuChoice {
    EasyMode,
    HardMode,
    Quit
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
            b'q' => {
                return MainMenuChoice::Quit;
            },
            _ => ()
        }
    }
}

