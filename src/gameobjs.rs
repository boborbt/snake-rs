use crate::renderable::{ Renderable, Frame };
use std::io::Write;


use termion::{
    color
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
            AppleType::Red => write!(stdout, "{}{}❤︎{}{:?}", self.frame.goto(self.x,self.y), color::Fg(color::Red), color::Fg(color::Reset), (self.x, self.y)).unwrap(),
            AppleType::Yellow => write!(stdout, "{}{}❦{}{:?}", self.frame.goto(self.x,self.y), color::Fg(color::Yellow), color::Fg(color::Reset), (self.x, self.y)).unwrap()
        }
    }
}

impl Apple {
    pub(crate) fn new(points:u64, speed:u64, apple_type: AppleType, frame: Frame) -> Apple {
        let (x,y) = frame.random_point();

        let apple_type = apple_type;
        let points = points;
        let inc_speed = speed;

        Apple { x, y, points, inc_speed, apple_type, frame: frame }
    }
}

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