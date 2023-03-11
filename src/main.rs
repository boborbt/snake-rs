use termion::{
    raw::IntoRawMode,
    clear,
    cursor,
    color,
    async_stdin,
    terminal_size
};

use std::{
    io::{ Read, Write, stdout },
    thread::sleep, time::Duration,
    time::Instant,
};

use rand;

trait Renderable<W: Write> {
    fn render(&self, stdout: &mut W);
}

enum AppleTypes {
    Red,
    Yellow
}

struct Apple {
    x: u16,
    y: u16,
    points: u64,
    inc_speed: u64,
    apple_type: AppleTypes
}


impl<W:Write> Renderable<W> for Apple {
    fn render(&self, stdout: &mut W) {
        match self.apple_type {
            AppleTypes::Red => write!(stdout, "{}{}●{}", cursor::Goto(self.x,self.y), color::Fg(color::Red), color::Fg(color::Reset)).unwrap(),
            AppleTypes::Yellow => write!(stdout, "{}{}●{}", cursor::Goto(self.x,self.y), color::Fg(color::Yellow), color::Fg(color::Reset)).unwrap()
        }
    }
}

struct CenteredPanel<'a> {
    content: Vec<&'a str>,
    field: (u16, u16)
}

impl<W:Write> Renderable<W> for CenteredPanel<'_> {
    fn render(&self, stdout: &mut W) {
        let mut row = (self.field.1 - self.content.len() as u16) / 2;
        for line in &self.content {
            let col = (self.field.0 - line.len() as u16) / 2;
            write!(stdout, "{}{}", cursor::Goto(col, row), line).unwrap();
            row += 1;
        }
    }
}

const GAME_OVER_SCREEN:[&str;5] =  ["+--------------------------------+" ,
                                    "|                                |" ,
                                    "|            GAME OVER           |" ,
                                    "|                                |" ,
                                    "+--------------------------------+"];

struct InfoPanel {
    score: u64,
    speed: u64,
    field: (u16, u16),
    char: u8
}

impl<W:Write> Renderable<W> for InfoPanel {
    fn render(&self, stdout: &mut W) {
        let dashes = (0..self.field.0).map(|_| "-").collect::<String>();
        let row = self.field.1 + 1;
        write!(stdout, "{}+{}+", cursor::Goto(1, row), dashes).unwrap();
        let row = row + 1;
        write!(stdout, "{}| {}Score{}: {} {}Speed{}: {} char: {}{}|", 
                cursor::Goto(1, row), 
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.score,
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                self.speed,
                self.char,
                cursor::Goto(self.field.0+2, row)
            ).unwrap();
        let row = row + 1;
        write!(stdout, "{}+{}+", cursor::Goto(1, row), dashes).unwrap();
    }
}

struct Snake {
    body: Vec<(u16, u16)>,
    dir: (i16, i16),
} 

impl Snake {
    fn mv(&mut self, field: &(u16, u16)) {
        let mut new_x = self.body[0].0 as i16 + self.dir.0;
        let mut new_y = self.body[0].1 as i16 + self.dir.1;

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

        self.body.insert(0, (new_x as u16, new_y as u16));
        self.body.pop();
    }

    fn head_pos(&self) -> (u16, u16) {
        self.body[0]
    }

    fn grow(&mut self) {
        let last = self.body.len() - 1;
        let last_pos = self.body[last];
        self.body.push(last_pos);
        self.body.push(last_pos);
    }
}

impl<W: Write> Renderable<W> for Snake {
    fn render(&self, stdout: &mut W) {
        let mut str: String = String::new();

        for (x,y) in &self.body {
            str.push_str(&String::from(cursor::Goto(*x,*y)));
            str.push('●');
        }

        write!(stdout, "{}{}{}", color::Fg(color::Green), str, color::Fg(color::Reset)).unwrap();
    }
}
struct App<R, W> {
    red_apple: Apple,
    yellow_apple: Apple,
    snake: Snake,
    stdin: R,
    stdout: W,
    speed: u64,
    field: (u16, u16),
    score: u64,
    game_over: bool,
    char: u8
}

impl<R: Read, W: Write>  App<R, W> {
    fn new(stdin: R, stdout: W) -> App<R,W> {
        let size = terminal_size().unwrap();
        let size = (size.0 - 2, size.1 - 4);
        App {
            red_apple: Apple { x:5, y:5, points: 1, inc_speed: 1, apple_type: AppleTypes::Red },
            yellow_apple: Apple { x:10, y:10, points: 2, inc_speed: 2, apple_type: AppleTypes::Yellow },
            snake: Snake { body: vec![(3,1),(2,1),(1,1)], dir: (1,0) },
            stdin: stdin,
            stdout: stdout,
            speed: 10,
            field: size,
            score: 0,
            game_over: false,
            char: ' ' as u8
        }
    }

    fn render(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();
        self.red_apple.render(&mut self.stdout);
        self.yellow_apple.render(&mut self.stdout);
        self.snake.render(&mut self.stdout);

        let info_panel = InfoPanel { score: self.score, speed: self.speed, field: self.field, char: self.char };
        info_panel.render(&mut self.stdout);

        self.stdout.flush().unwrap();
    }

    fn check_collision(&mut self) {
        let head_pos = self.snake.head_pos();
        let mut apple_eaten = false;

        for apple in [&mut self.red_apple, &mut self.yellow_apple].iter() {
            if head_pos.0 == apple.x && head_pos.1 == apple.y {
                self.snake.grow();
                self.speed += apple.inc_speed;
                self.score += apple.points;
                apple_eaten = true;
            }
        }

        if apple_eaten {
            self.red_apple.x = rand::random::<u16>() % self.field.0 + 1;
            self.red_apple.y = rand::random::<u16>() % self.field.1 + 1;
            self.yellow_apple.x = rand::random::<u16>() % self.field.0 + 1;
            self.yellow_apple.y = rand::random::<u16>() % self.field.1 + 1;
        }


        for (x,y) in &self.snake.body[1..] {
            if head_pos.0 == *x && head_pos.1 == *y {
                self.game_over = true;
            }
        }
    }


    fn run(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Hide).unwrap();

        let mut before = Instant::now();
        loop {
            let mut key_bytes = [0];
            self.stdin.read(&mut key_bytes).unwrap();

            if key_bytes[0] != 0 {
                self.char = key_bytes[0];
            }

            match key_bytes[0] {
                27 => {
                    self.stdin.read(&mut key_bytes).unwrap();
                    self.stdin.read(&mut key_bytes).unwrap();
                    self.char = key_bytes[0];
                    match key_bytes[0] {
                        65 => self.snake.dir = (0, -1),
                        66 => self.snake.dir = (0, 1),
                        67 => self.snake.dir = (1, 0),
                        68 => self.snake.dir = (-1, 0),
                        _ => {}
                    }
                }
                b'q' => break,
                b'w' => self.snake.dir = (0, -1),
                b'a' => self.snake.dir = (-1, 0),
                b's' => self.snake.dir = (0, 1),
                b'd' => self.snake.dir = (1, 0),
                _ => {}
            }

            let mut speed = self.speed;
            if self.snake.dir.1 != 0 {
                speed = (speed as f32 / 1.6) as u64;
            }

            let interval = 1000 / speed;
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                sleep(Duration::from_millis(interval - dt));
                continue;
            }

            before = now;

            self.snake.mv(&self.field);
            self.check_collision();
            self.render();

            if self.game_over {
                let cp = CenteredPanel {
                    content: Vec::from(GAME_OVER_SCREEN),
                    field: self.field                    
                };

                cp.render(&mut self.stdout);
                write!(self.stdout, "{}", cursor::Goto(1, self.field.1+4)).unwrap();
                break;
            }
        }

        write!(self.stdout, "{}", cursor::Show).unwrap();
    }
}

fn main() {
    let mut stdout = stdout();
    let mut stdin = async_stdin();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    stdout.activate_raw_mode().unwrap();
    
    let mut app = App::new(&mut stdin, &mut stdout);

    app.run();
}
