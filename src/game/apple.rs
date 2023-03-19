use crate::io::renderable::{ Renderable, Frame };
use std::io::Write;
use termion::color;

#[derive(Clone,PartialEq)]
pub(crate) enum AppleType {
    Red,
    Yellow
}

#[derive(Clone)]
pub(crate) struct Apple {
    pub(crate) pos: (u16, u16),
    pub(crate) points: u64,
    pub(crate) inc_speed: u64,
    pub(crate) apple_type: AppleType,
    pub(crate) frame: Frame
}


impl Renderable for Apple {
    fn render<W:Write>(&self, stdout: &mut W) {
        match self.apple_type {
            AppleType::Red => write!(stdout, "{}{}❤︎{}", self.frame.goto(self.pos.0,self.pos.1), color::Fg(color::Red), color::Fg(color::Reset)).unwrap(),
            AppleType::Yellow => write!(stdout, "{}{}❦{}", self.frame.goto(self.pos.0,self.pos.1), color::Fg(color::Yellow), color::Fg(color::Reset)).unwrap()
        }
    }
}

impl Apple {
    pub(crate) fn new(points:u64, speed:u64, apple_type: AppleType, frame: Frame) -> Apple {
        let pos = frame.random_point();

        let apple_type = apple_type;
        let points = points;
        let inc_speed = speed;

        Apple { pos, points, inc_speed, apple_type, frame: frame }
    }
}