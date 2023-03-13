// Snake implementation in Rust
// Copyright (C) 2023  Roberto Esposito

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


use termion::{
    raw::IntoRawMode,
    clear,
    cursor,
    color,
    async_stdin,
    terminal_size, AsyncReader
};

use std::{
    env,
    io::{ Read, Write, stdout },
    thread::sleep, time::Duration,
    time::Instant, ops::ControlFlow,
};

use rand;

trait Renderable {
    fn render<W: Write>(&self, stdout: &mut W);
}

#[derive(Clone)]
enum AppleType {
    Red,
    Yellow
}

enum Command {
    Up,
    Down,
    Left,
    Right,
    Quit,
    None
}

#[derive(Clone)]
struct Apple {
    x: u16,
    y: u16,
    points: u64,
    inc_speed: u64,
    apple_type: AppleType
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
    fn new(field: &(u16, u16), points:u64, speed:u64, apple_type: AppleType) -> Apple {
        let x: u16 = rand::random::<u16>() % field.0 + 1;
        let y: u16 = rand::random::<u16>() % field.1 + 1;


        let apple_type = apple_type;
        let points = points;
        let inc_speed = speed;

        Apple { x, y, points, inc_speed, apple_type }
    }
}

struct CenteredPanel<'a> {
    content: Vec<&'a str>,
    field: (u16, u16)
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

const GAME_OVER_SCREEN:[&str;5] =  ["╭────────────────────────────────╮" ,
                                    "│                                │" ,
                                    "│            GAME OVER           │" ,
                                    "│                                │" ,
                                    "╰────────────────────────────────╯"];

#[derive(Clone)]
struct InfoPanel {
    score: u64,
    speed: u64,
    field: (u16, u16)
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

#[derive(Clone)]
struct Snake {
    body: Vec<(u16, u16)>,
    dir: (i16, i16),
} 

impl Snake {
    fn mv(&self, field: &(u16, u16)) -> Snake {
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
            new_y = field.1 as i16 ;
        }

        if new_y > field.1 as i16{
            new_y = 1;
        }

        snake.body.insert(0, (new_x as u16, new_y as u16));
        snake.body.pop();

        snake
    }

    fn head_pos(&self) -> (u16, u16) {
        self.body[0]
    }

    fn grow(&self) -> Snake {
        let mut snake = self.clone();
        let last = self.body.len() - 1;
        let last_pos = self.body[last].clone();
        snake.body.push(last_pos);
        snake.body.push(last_pos);
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

#[derive(Clone)]
struct App {
    red_apple: Apple,
    yellow_apple: Apple,
    snake: Snake,
    speed: u64,
    field: (u16, u16),
    score: u64,
    game_over: bool,
    easy_mode: bool
}

impl App {
    fn new(easy_mode: bool) -> App {
        let result = App {
            red_apple: Apple { x:5, y:5, points: 1, inc_speed: 1, apple_type: AppleType::Red },
            yellow_apple: Apple { x:10, y:10, points: 2, inc_speed: 2, apple_type: AppleType::Yellow },
            snake: Snake { body: vec![(3,1),(2,1),(1,1)], dir: (1,0) },
            speed: 10,
            field: (80,25),
            score: 0,
            game_over: false,
            easy_mode: easy_mode
        };

        result.update_field_size()
    }

    fn update_field_size(&self) -> App {
        let mut result = self.clone();
        let size = terminal_size().unwrap();
        let size = (size.0 - 2, size.1 - 4);

        result.field = size;
        result
    }

    fn render<W:Write>(&self, stdout: &mut W) {
        write!(stdout, "{}", clear::All).unwrap();
        self.red_apple.render(stdout);
        self.yellow_apple.render(stdout);
        self.snake.render(stdout);

        let info_panel = InfoPanel { score: self.score, speed: self.speed, field: self.field };
        info_panel.render(stdout);

        stdout.flush().unwrap();
    }

    fn check_collision(&self) -> App {
        let mut result = self.clone();
        let head_pos = self.snake.head_pos();
        let mut apple_eaten = false;

        for apple in [&self.red_apple,&self.yellow_apple].iter() {
            if head_pos.0 == apple.x && head_pos.1 == apple.y {
                result.snake = self.snake.grow();
                result.speed += apple.inc_speed;
                result.score += apple.points;
                apple_eaten = true;
            }
        }

        if apple_eaten {
            result.red_apple = Apple::new(&self.field, 1, 1, AppleType::Red);
            result.yellow_apple = Apple::new(&self.field, 2, 2, AppleType::Yellow);
        }


        for (x,y) in &self.snake.body[1..] {
            if head_pos.0 == *x && head_pos.1 == *y {
                result.game_over = true;
            }
        }

        result
    }

    fn get_cmd(stdin: &mut AsyncReader) -> Command {
            let mut key_bytes = [0];
            stdin.read(&mut key_bytes).unwrap();

            match key_bytes[0] {
                27 => {
                    stdin.read(&mut key_bytes).unwrap();
                    stdin.read(&mut key_bytes).unwrap();
                    match key_bytes[0] {
                        65 => return Command::Down,
                        66 => return Command::Up,
                        67 => return Command::Right,
                        68 => return Command::Left,
                        _ => return Command::None
                    }
                }

                b'q' => return Command::Quit,
                b'w' => return Command::Down,
                b's' => return Command::Up,
                b'a' => return Command::Left,
                b'd' => return Command::Right,
                _ => return Command::None
            }
    }

    fn react_to_command(&self, cmd: Command) -> App {
        let mut result = self.clone();
        let mut newdir = (0,0);

        match cmd {
            Command::Quit => result.game_over = true,
            Command::Up     => newdir = (0,1),
            Command::Down   => newdir = (0,-1),
            Command::Left   => newdir = (-1,0),
            Command::Right  => newdir = (1,0),
            Command::None   => {}
        }

        if newdir == (0,0) {
            return result;
        }

        if !self.easy_mode {
            result.snake.dir = newdir;
        } else if newdir.0 != -self.snake.dir.0 && newdir.1 != -self.snake.dir.1 {
            result.snake.dir = newdir;
        }

        result
    }

    fn show_game_over_message<W: Write>(&self, stdout: &mut W) {
        let cp = CenteredPanel {
            content: Vec::from(GAME_OVER_SCREEN),
            field: self.field                    
        };

        cp.render(stdout);
        write!(stdout, "{}", cursor::Goto(1, self.field.1+4)).unwrap();
    }

    fn wait_next_turn(&self, now: Instant, before: Instant) -> ControlFlow<()> {
        let mut speed = self.speed;
        
        if self.snake.dir.1 != 0 {
            speed = (speed as f32 / 1.6) as u64;
        }

        let interval = 1000 / speed;
        let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;


        if dt < interval {
            sleep(Duration::from_millis(interval - dt));
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }

    fn run<W:Write>(stdin: &mut AsyncReader, stdout: &mut W, easy_mode: bool) {
        let mut app = App::new(easy_mode);
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        let mut before = Instant::now();
        loop {
            app = app.update_field_size();
  
            let now = Instant::now();

            if let ControlFlow::Break(_) = app.wait_next_turn(now, before) {
                continue;
            }

            before = now;

            app = app.react_to_command(App::get_cmd(stdin));
            
            if app.game_over {
                break;
            }

            app.snake = app.snake.mv(&app.field);
            app = app.check_collision();
            app.render(stdout);

            if app.game_over {
                app.show_game_over_message(stdout);
                break;
            }
        }

        write!(stdout, "{}{}", cursor::Goto(1, app.field.1+4), cursor::Show).unwrap();
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let easy_mode = args.len() > 1 && args[1] == "--easy";
        

    let stdout = stdout();
    let mut stdin = async_stdin();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    stdout.activate_raw_mode().unwrap();
    
    App::run(&mut stdin, &mut stdout, easy_mode);
}
