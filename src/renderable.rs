use std::io::Write;
use termion::{
    cursor,
    color,
    AsyncReader
};
use crate::io::wait_char;


pub(crate) trait Renderable {
    fn render<W: Write>(&self, stdout: &mut W);
}

#[derive(Clone)]
pub(crate) struct CenteredPanel<'a> {
    pub(crate) content: Vec<&'a str>,
    pub(crate) field: (u16, u16)
}

impl Renderable for CenteredPanel<'_> {
    fn render<W:Write>(&self, stdout: &mut W) {
        let mut row = (self.field.1 - self.content.len() as u16) / 2;
        for line in &self.content {
            let col = (self.field.0 - line.chars().count() as u16) / 2;
            write!(stdout, "{}{}", cursor::Goto(col, row), line).unwrap();
            row += 1;
        }
    }
}

pub(crate) const GAME_OVER_SCREEN:[&str;5] =  ["╭────────────────────────────────╮" ,
                                               "│                                │" ,
                                               "│            GAME OVER           │" ,
                                               "│                                │" ,
                                               "╰────────────────────────────────╯"];

pub(crate) const MAIN_MENU_SCREEN:[&str;8] =  ["╭─────────────────────────────╮" ,
                                               "│            SNAKE            │" ,
                                               "│                             │" ,
                                               "│        1. EASY MODE         │" ,
                                               "│        2. HARD MODE         │" ,
                                               "│        q. QUIT              │" ,
                                               "│                             │" ,
                                               "╰─────────────────────────────╯"];

pub(crate) const CONFIRM_QUIT_SCREEN:[&str;6] =  ["╭─────────────────────────────╮" ,
                                                  "│                             │" ,
                                                  "│  Confirm quitting the game? │" ,
                                                  "│             y/N             │" ,
                                                  "│                             │" ,
                                                  "╰─────────────────────────────╯"];

#[derive(Clone)]
pub(crate) struct InfoPanel {
    pub(crate) score: u64,
    pub(crate) speed: u64,
    pub(crate) field: (u16, u16)
}

impl Renderable for InfoPanel {
    fn render<W:Write>(&self, stdout: &mut W) {
        let dashes = (0..self.field.0).map(|_| "─").collect::<String>();
        let row = self.field.1 + 1;
        write!(stdout, "{}╭{}╮", cursor::Goto(1, row), dashes).unwrap();
        let row = row + 1;
        write!(stdout, "{}│ {}Score{}: {} {}Speed{}: {}{}│", 
                cursor::Goto(1, row), 
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.score,
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.speed,
                cursor::Goto(self.field.0+2, row)
            ).unwrap();
        let row = row + 1;
        write!(stdout, "{}╰{}╯", cursor::Goto(1, row), dashes).unwrap();
    }
}

pub(crate) fn confirm_quit<W:Write>(stdin: &mut AsyncReader, stdout: &mut W, field:(u16, u16)) -> bool {
    let confirm_dialog = CenteredPanel {
        content: Vec::from(CONFIRM_QUIT_SCREEN),
        field: field
    };

    confirm_dialog.render(stdout);
    stdout.flush().unwrap();

    let choice:u8 = wait_char(stdin);

    return choice == b'y';
}

