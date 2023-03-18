use std::io::Write;
use termion::{
    cursor,
    color,
    AsyncReader,
};
use crate::io::wait_char;

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Frame {
    pub(crate) pos: (u16, u16),
    pub(crate) size: (u16, u16)
}

impl Frame {
    pub(crate) fn new(pos: (u16, u16), size: (u16, u16)) -> Frame {
        Frame { pos, size }
    }

    pub(crate) fn render<W:Write>(&self, stdout: &mut W) {
        let (x, y) = self.pos;
        let (w, h) = self.size;

        write!(stdout, "{}╭{}╮", cursor::Goto(x, y), "─".repeat((w - 2) as usize)).unwrap();

        for i in 1..(h-1) {
            write!(stdout, "{}│{}│", cursor::Goto(x, y + i), cursor::Goto(x+w-1,y+i)).unwrap();
        }
        write!(stdout, "{}╰{}╯", cursor::Goto(x, y + h-1), "─".repeat((w - 2) as usize)).unwrap();
    }

    pub(crate) fn goto(&self, x: u16, y: u16) -> cursor::Goto {
        let (_x, _y) = self.pos;
        return cursor::Goto(_x + x, _y + y)
    }

    pub(crate) fn field(&self) -> (u16, u16) {
        let (x, y) = self.pos;
        let (w, h) = self.size;
        
        (w - x - 1, h - y - 1)
    }

    pub(crate) fn random_point(&self) -> (u16, u16) {
        let (w, h) = self.field();
        let x: u16 = rand::random::<u16>() % w + 1;
        let y: u16 = rand::random::<u16>() % h + 1;

        (x, y)
    }
}


pub(crate) trait Renderable {
    fn render<W: Write>(&self, stdout: &mut W);
}

#[derive(Clone)]
pub(crate) struct CenteredPanel<'a> {
    pub(crate) content: Vec<&'a str>,
    pub(crate) frame: Frame
}

impl Renderable for CenteredPanel<'_> {
    fn render<W:Write>(&self, stdout: &mut W) {
        let mut row = (self.frame.size.1 - self.content.len() as u16) / 2;
        for line in &self.content {
            let col = (self.frame.size.0 - line.chars().count() as u16) / 2;
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

pub(crate) const MAIN_MENU_SCREEN:[&str;10] =  ["╭─────────────────────────────╮" ,
                                                "│            SNAKE            │" ,
                                                "│                             │" ,
                                                "│      1. EASY MODE           │" ,
                                                "│      2. HARD MODE           │" ,
                                                "│      3. EASY MODE 80x25     │" ,
                                                "│      4. HARD MODE 80x25     │" ,
                                                "│      q. QUIT                │" ,
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
    pub(crate) frame: Frame
}

impl Renderable for InfoPanel {
    fn render<W:Write>(&self, stdout: &mut W) {
        self.frame.render(stdout);
        write!(stdout, "{}{}Score{}: {} {}Speed{}: {}", 
                self.frame.goto(2, 1), 
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.score,
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.speed
            ).unwrap();
    }
}

pub(crate) fn confirm_quit<W:Write>(stdin: &mut AsyncReader, stdout: &mut W, frame: Frame) -> bool {
    let confirm_dialog = CenteredPanel {
        content: Vec::from(CONFIRM_QUIT_SCREEN),
        frame
    };

    confirm_dialog.render(stdout);
    stdout.flush().unwrap();

    let choice:u8 = wait_char(stdin);

    return choice == b'y';
}

