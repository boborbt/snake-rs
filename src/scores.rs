use termion::{color, terminal_size};
use std::io::{Read, Write};
use crate::{renderable::Renderable, menu::MainMenuChoice};
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Difficulty {
    Easy,
    Hard
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct LBScore {
    pub(crate) last: u64,
    pub(crate) best: u64
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct ScoreBoard {
    scores: [(MainMenuChoice, LBScore); 4]
}

impl Renderable for ScoreBoard {
    fn render<W:Write>(&self, stdout: &mut W) {
        let max_label_len = MainMenuChoice::HardMode80x25.to_string().len()+1;
        let bwstr = format!("{:max_label_len$} [last: {:>4} | best: {:>4}]", "", 0,0);
        let margin = String::from_utf8(vec![b' '; (terminal_size().unwrap().0 as usize - bwstr.len()) / 2]).unwrap();

        for score in self.scores {
            let str = format!("{}{:max_label_len$}{}: [{}last{}: {:>4} | {}best{}: {:>4}]",
                color::Fg(color::Red),
                score.0.to_string(),
                color::Fg(color::Reset),
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                score.1.last.to_string(),
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                score.1.best.to_string());
                write!(stdout, "{}{}{}\n\r", margin, str, margin).unwrap();
        }
    }
}

impl ScoreBoard {
    pub(crate) fn new() -> ScoreBoard {
        ScoreBoard {
            scores: [
                (MainMenuChoice::EasyMode, LBScore { last: 0, best: 0 }),
                (MainMenuChoice::HardMode, LBScore { last: 0, best: 0 }),
                (MainMenuChoice::EasyMode80x25, LBScore { last: 0, best: 0 }),
                (MainMenuChoice::HardMode80x25, LBScore { last: 0, best: 0 })
            ]
        }
    }

    pub(crate) fn update(self, score: u64, choice: MainMenuChoice) -> ScoreBoard {
        let mut new_scores = self.scores;
        let elem = new_scores.iter_mut().find(|(c,_)| *c == choice).unwrap();
        elem.1.last = score;
        elem.1.best = std::cmp::max(elem.1.best, elem.1.last);

        return ScoreBoard { scores: new_scores };
    }

    pub(crate) fn load() -> ScoreBoard {
        let file = std::fs::File::open("scores.json");
        if let Ok(mut file) = file {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let scores: ScoreBoard = serde_json::from_str(&contents).unwrap();
            return scores;
        }

        return ScoreBoard::new();
    }

    pub(crate) fn save(self)  {
        let mut file = std::fs::File::create("scores.json").unwrap();    

        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

}