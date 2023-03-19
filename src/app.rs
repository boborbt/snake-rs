use crate::{
    gameobjs::{
        Apple,
        AppleType,
        Snake
    },
    io::wait_char,
    renderable::{
        Renderable,
        InfoPanel,
        CenteredPanel,
        GAME_OVER_SCREEN,
        confirm_quit,
        Frame
    }
};

use termion::{
    clear,
    cursor,
    terminal_size, 
    AsyncReader
};

use core::ops::ControlFlow;
use std::{
    io::{ Read, Write },
    time::{ Instant, Duration },
    thread::sleep
};

enum Command {
    Up,
    Down,
    Left,
    Right,
    Quit,
    None
}


#[derive(Clone)]
pub(crate) struct App {
    frame: Frame,
    red_apple: Apple,
    yellow_apple: Apple,
    snake: Snake,
    speed: u64,
    score: u64,
    game_over: bool,
    quit: bool,
    easy_mode: bool,
    fixed_size: bool
}

impl App {
    fn new(easy_mode: bool, size:Option<(u16, u16)>) -> App {
        let frame = match size {
            Some(size) => Frame { pos:(1,1), size },
            None => Frame { pos:(1,1), size: (78,23) }
        }
        ;
        let result = App {
            frame: frame,
            red_apple: Apple { pos:frame.random_point(), points: 1, inc_speed: 1, apple_type: AppleType::Red, frame: frame },
            yellow_apple: Apple { pos:frame.random_point(), points: 2, inc_speed: 2, apple_type: AppleType::Yellow, frame: frame },
            snake: Snake { body: vec![(3,1),(2,1),(1,1)], dir: (1,0), frame },
            speed: 10,
            score: 0,
            game_over: false,
            quit: false,
            easy_mode: easy_mode,
            fixed_size: size.is_some()
        };

        result.update_frame_size()
    }

    fn update_frame_size(self) -> App {
        if self.fixed_size {
            return self;
        }

        let size = terminal_size().unwrap();
        let size = (size.0, size.1 - 3);
        let frame = Frame { pos: (1,1), size };

        if frame == self.frame {
            return self;
        }

        App {
            frame,
            red_apple: Apple { pos:frame.random_point(), frame, ..self.red_apple },
            yellow_apple: Apple { pos:frame.random_point(), ..self.yellow_apple },
            snake: Snake { frame, ..self.snake },
            ..self
        }
    }

    fn render<W:Write>(&self, stdout: &mut W) {
        write!(stdout, "{}", clear::All).unwrap();
        self.frame.render(stdout);
        self.red_apple.render(stdout);
        self.yellow_apple.render(stdout);
        self.snake.render(stdout);

        let info_panel_frame = Frame { pos: (self.frame.pos.0, self.frame.pos.1 + self.frame.size.1), size: (self.frame.size.0, 3) };
        let info_panel = InfoPanel { score: self.score, speed: self.speed, frame: info_panel_frame };
        info_panel.render(stdout);

        stdout.flush().unwrap();
    }

    fn check_collision(&self) -> App {
        let mut result = self.clone();
        let head_pos = self.snake.head_pos();
        let mut apple_eaten: Option<AppleType> = None;

        for apple in [&self.red_apple,&self.yellow_apple].iter() {
            if head_pos.0 == apple.pos.0 && head_pos.1 == apple.pos.1 {
                result.snake = self.snake.grow(apple.points as u16);
                result.speed += apple.inc_speed;
                result.score += apple.points;
                apple_eaten = Some(apple.apple_type.clone());
            }
        }

        if Some(AppleType::Red) == apple_eaten {
            result.red_apple = Apple::new(1, 1, AppleType::Red, self.frame);
        }

        if Some(AppleType::Yellow) == apple_eaten {
            result.yellow_apple = Apple::new(2, 2, AppleType::Yellow, self.frame);
        }

        for (x,y) in &self.snake.body[1..] {
            if head_pos.0 == *x && head_pos.1 == *y {
                result.game_over = true;
            }
        }

        result
    }

    fn input_cmd(stdin: &mut AsyncReader) -> Command {
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
            Command::Quit => result.quit = true,
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
            frame: self.frame                    
        };

        cp.render(stdout);
        write!(stdout, "{}", cursor::Goto(1, self.frame.size.1+4)).unwrap();
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
            return ControlFlow::Continue(());
        }
        ControlFlow::Break(())
    }

    pub(crate) fn run<W:Write>(stdin: &mut AsyncReader, stdout: &mut W, easy_mode: bool, size: Option<(u16, u16)>) -> u64 {
        let mut app = App::new(easy_mode, size);
        let mut before = Instant::now();
        loop {
            app = app.update_frame_size();
  
            let now = Instant::now();

            if let ControlFlow::Continue(_) = app.wait_next_turn(now, before) {
                continue;
            }

            before = now;

            app = app.react_to_command(App::input_cmd(stdin));

            app.snake = app.snake.mv();
            app = app.check_collision();
            app.render(stdout);

            if app.quit && confirm_quit(stdin, stdout, app.frame) {
                break;
            } else {
                app.quit = false;
            }

            if app.game_over {
                app.show_game_over_message(stdout);
                stdout.flush().unwrap();
                wait_char(stdin);
                break;
            }
        }

        return app.score;
    }

}