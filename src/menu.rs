use std::io::{Read, Write};

use termion::{ AsyncReader, terminal_size };

use crate::renderable::{
    Renderable,
    CenteredPanel,
    MAIN_MENU_SCREEN
};

pub(crate) enum MainMenuChoice {
    EasyMode,
    HardMode,
    Quit
}

pub(crate) fn run<W:Write>(stdin:&mut AsyncReader, stdout:&mut W) -> MainMenuChoice {
    let panel = CenteredPanel {
        content: MAIN_MENU_SCREEN.to_vec(),
        field: terminal_size().unwrap()
    };
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
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
    }
}

