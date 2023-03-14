use termion::{color, terminal_size};
use std::io::Write;
use crate::renderable::Renderable;

pub(crate) enum Difficulty {
    Easy,
    Hard
}

#[derive(Copy, Clone)]
pub(crate) struct ScoreBoard {
    pub(crate) last_easy_score: u64,
    pub(crate) best_easy_score: u64,
    pub(crate) last_hard_score: u64,
    pub(crate) best_hard_score: u64
}

impl Renderable for ScoreBoard {
    fn render<W:Write>(&self, stdout: &mut W) {
        let bwstr = format!("Easy scores: [last: {:>4} | best: {:>4}]",
                                self.last_easy_score.to_string(),
                                self.best_easy_score.to_string());

        let margin = String::from_utf8(vec![b' '; (terminal_size().unwrap().0 as usize - bwstr.len()) / 2]).unwrap();

        let easy_str = format!("{}Easy scores{}: [{}last{}: {:>4} | {}best{}: {:>4}]",
            color::Fg(color::Red),
            color::Fg(color::Reset),
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            self.last_easy_score.to_string(),
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            self.best_easy_score.to_string());

        let hard_str = format!("{}Hard scores{}: [{}last{}: {:>4} | {}best{}: {:>4}]",
            color::Fg(color::Red),
            color::Fg(color::Reset),
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            self.last_hard_score.to_string(),
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            self.best_hard_score.to_string());

        write!(stdout, "{}{}{}\n\r", margin, easy_str, margin).unwrap();
        write!(stdout, "{}{}{}\n\r", margin, hard_str, margin).unwrap();
    }
}

impl ScoreBoard {
    pub(crate) fn new() -> ScoreBoard {
        ScoreBoard {
            last_easy_score: 0,
            best_easy_score: 0,
            last_hard_score: 0,
            best_hard_score: 0
        }
    }

    pub(crate) fn update(self, score: u64, difficulty: Difficulty) -> ScoreBoard {
        match difficulty {
            Difficulty::Easy => {
                ScoreBoard {
                    last_easy_score: score,
                    best_easy_score: std::cmp::max(self.best_easy_score, score),
                    ..self
                }
            },
            Difficulty::Hard => {
                ScoreBoard {
                    last_hard_score: score,
                    best_hard_score: std::cmp::max(self.best_hard_score, score),
                    ..self
                }
            }
        }
    }
}