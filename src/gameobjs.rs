use crate::renderable::Renderable;
use std::io::Write;


use termion::{
    cursor,
    color,
};


#[derive(Clone)]
enum AppleType {
    Red,
    Yellow
}

pub struct Apple {
    pub x: u16,
    pub y: u16,
    pub points: u64,
    pub inc_speed: u64,
    pub apple_type: AppleType
}


impl Renderable for Apple {
    fn render<W:Write>(&self, stdout: &mut W) {
        match self.apple_type {
            AppleType::Red => write!(stdout, "{}{}❤︎{}", cursor::Goto(self.x,self.y), color::Fg(color::Red), color::Fg(color::Reset)).unwrap(),
            AppleType::Yellow => write!(stdout, "{}{}❦{}", cursor::Goto(self.x,self.y), color::Fg(color::Yellow), color::Fg(color::Reset)).unwrap()
        }
    }
}

impl Apple {
    pub fn new(field: &(u16, u16), points:u64, speed:u64, apple_type: AppleType) -> Apple {
        let x: u16 = rand::random::<u16>() % field.0 + 1;
        let y: u16 = rand::random::<u16>() % field.1 + 1;


        let apple_type = apple_type;
        let points = points;
        let inc_speed = speed;

        Apple { x, y, points, inc_speed, apple_type }
    }
}