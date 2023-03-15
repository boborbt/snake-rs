use crate::renderable::{ Renderable, Frame };
use std::io::Write;


use termion::{
    cursor,
    color,
};


#[derive(Clone,PartialEq)]
pub(crate) enum AppleType {
    Red,
    Yellow
}

#[derive(Clone)]
pub(crate) struct Apple {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) points: u64,
    pub(crate) inc_speed: u64,
    pub(crate) apple_type: AppleType,
    pub(crate) frame: Frame
}


impl Renderable for Apple {
    fn render<W:Write>(&self, stdout: &mut W) {
        match self.apple_type {
            AppleType::Red => write!(stdout, "{}{}❤︎{}", self.frame.goto(self.x,self.y), color::Fg(color::Red), color::Fg(color::Reset)).unwrap(),
            AppleType::Yellow => write!(stdout, "{}{}❦{}", self.frame.goto(self.x,self.y), color::Fg(color::Yellow), color::Fg(color::Reset)).unwrap()
        }
    }
}

impl Apple {
    pub(crate) fn new(field: &(u16, u16), points:u64, speed:u64, apple_type: AppleType, frame: Frame) -> Apple {
        let x: u16 = rand::random::<u16>() % field.0 + 1;
        let y: u16 = rand::random::<u16>() % field.1 + 1;


        let apple_type = apple_type;
        let points = points;
        let inc_speed = speed;

        Apple { x, y, points, inc_speed, apple_type, frame }
    }
}

#[derive(Clone)]
pub(crate) struct Snake {
    pub(crate) body: Vec<(u16, u16)>,
    pub(crate) dir: (i16, i16),
} 

impl Snake {
    pub(crate) fn mv(&self, field: &(u16, u16)) -> Snake {
        let mut new_x = self.body[0].0 as i16 + self.dir.0;
        let mut new_y = self.body[0].1 as i16 + self.dir.1;
        let mut snake = self.clone();

        if new_x < 1 {
            new_x = field.0 as i16;
        }
        
        if new_x > field.0 as i16 {
            new_x = 1;
        }

        if new_y < 1 {
            new_y = field.1 as i16;
        }

        if new_y > field.1 as i16 {
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
            str.push_str(&String::from(cursor::Goto(*x,*y)));
            str.push('✿');
        }

        write!(stdout, "{}{}{}", color::Fg(color::Green), str, color::Fg(color::Reset)).unwrap();
    }
}