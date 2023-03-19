use crate::io::renderable::{ Renderable, Frame };
use std::io::Write;

use termion::{
    color
};

#[derive(Clone)]
pub(crate) struct Snake {
    pub(crate) body: Vec<(u16, u16)>,
    pub(crate) dir: (i16, i16),
    pub(crate) frame: Frame
} 

impl Snake {
    pub(crate) fn mv(&self) -> Snake {
        let mut new_x = self.body[0].0 as i16 + self.dir.0;
        let mut new_y = self.body[0].1 as i16 + self.dir.1;
        let mut snake = self.clone();

        if new_x < 1 {
            new_x = self.frame.field().0 as i16;
        }
        
        if new_x > self.frame.field().0 as i16 {
            new_x = 1;
        }

        if new_y < 1 {
            new_y = self.frame.field().1 as i16;
        }

        if new_y > self.frame.field().1 as i16 {
            new_y = 1;
        }

        snake.body.insert(0, (new_x as u16, new_y as u16));
        snake.body.pop();

        snake
    }

    pub(crate) fn head_pos(&self) -> (u16, u16) {
        self.body[0]
    }

    pub(crate) fn grow(&self, len: u16) -> Snake {
        let mut snake = self.clone();
        let last = self.body.len() - 1;
        let last_pos = self.body[last].clone();
        for _ in 0..len {
            snake.body.push(last_pos);
        }

        snake
    }
}

impl Renderable for Snake {
    fn render<W: Write>(&self, stdout: &mut W) {
        let mut str: String = String::new();

        for (x,y) in &self.body {
            str.push_str(&String::from(self.frame.goto(*x,*y)));
            str.push('✿');
        }

        write!(stdout, "{}{}{}", color::Fg(color::Green), str, color::Fg(color::Reset)).unwrap();
    }
}