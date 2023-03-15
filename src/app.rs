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
        confirm_quit
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
    red_apple: Apple,
    yellow_apple: Apple,
    snake: Snake,
    speed: u64,
    field: (u16, u16),
    score: u64,
    game_over: bool,
    quit: bool,
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
            quit: false,
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
        let mut apple_eaten: Option<AppleType> = None;

        for apple in [&self.red_apple,&self.yellow_apple].iter() {
            if head_pos.0 == apple.x && head_pos.1 == apple.y {
                result.snake = self.snake.grow(apple.points as u16);
                result.speed += apple.inc_speed;
                result.score += apple.points;
                apple_eaten = Some(apple.apple_type.clone());
            }
        }

        if Some(AppleType::Red) == apple_eaten {
            result.red_apple = Apple::new(&self.field, 1, 1, AppleType::Red);
        }

        if Some(AppleType::Yellow) == apple_eaten {
            result.yellow_apple = Apple::new(&self.field, 2, 2, AppleType::Yellow);
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

    pub(crate) fn run<W:Write>(stdin: &mut AsyncReader, stdout: &mut W, easy_mode: bool) -> u64 {
        let mut app = App::new(easy_mode);
        let mut before = Instant::now();
        loop {
            app = app.update_field_size();
  
            let now = Instant::now();

            if let ControlFlow::Break(_) = app.wait_next_turn(now, before) {
                continue;
            }

            before = now;

            app = app.react_to_command(App::input_cmd(stdin));

            app.snake = app.snake.mv(&app.field);
            app = app.check_collision();
            app.render(stdout);

            if app.quit && confirm_quit(stdin, stdout, app.field) {
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