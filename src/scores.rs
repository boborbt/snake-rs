use termion::{color, terminal_size, cursor};
use std::io::{Read, Write};
use crate::{renderable::Renderable, menu::{MenuAction, Difficulty}};
use serde::{Deserialize, Serialize};




#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct LBScore {
    pub(crate) last: u64,
    pub(crate) best: u64
}

#[derive(Copy, Clone, Serialize, Deserialize)]

pub(crate) struct Score {
    pub(crate) score: LBScore,
    pub(crate) difficulty: Difficulty,
    pub(crate) size: Option<(u16, u16)>
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct ScoreBoard {
    scores: [Score; 4]
}

impl Renderable for ScoreBoard {
    fn render<W:Write>(&self, stdout: &mut W) {
        let max_label_len = Difficulty::Hard.to_string().len() + " XXXxXXX".len() +1;
        let bwstr = format!("{:max_label_len$} [last: {:>4} | best: {:>4}]", "", 0,0);
        let margin = ((terminal_size().unwrap().0 as usize - bwstr.len()) / 2) as u16;

        for (index, elem) in self.scores.iter().enumerate() {
            let label = match elem.size {
                Some((w,h)) => format!("{}x{}", w, h),
                None => "Full".to_string()
            };
            
            let label = format!("{} {}", elem.difficulty.to_string(), label);

            let str = format!("{}{:max_label_len$}{}: [{}last{}: {:>4} | {}best{}: {:>4}]",
                color::Fg(color::Red),
                label,
                color::Fg(color::Reset),
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                elem.score.last.to_string(),
                color::Fg(color::Yellow),
                color::Fg(color::Reset),
                elem.score.best.to_string());
                write!(stdout, "{}{}\n\r", cursor::Goto(margin, (index+1) as u16), str).unwrap();
        }
    }
}

impl ScoreBoard {
    pub(crate) fn new() -> ScoreBoard {
        ScoreBoard {
            scores: [
                Score { score: LBScore { last: 0, best: 0 }, difficulty: Difficulty::Easy, size: None },
                Score { score: LBScore { last: 0, best: 0 }, difficulty: Difficulty::Hard, size: None },
                Score { score: LBScore { last: 0, best: 0 }, difficulty: Difficulty::Easy, size: Some((80,25)) },
                Score { score: LBScore { last: 0, best: 0 }, difficulty: Difficulty::Hard, size: Some((80,25)) }
            ]
        }
    }

    pub(crate) fn update(self, score: u64, choice: MenuAction) -> ScoreBoard {
        let (difficulty, size) = match choice {
            MenuAction::StartGame(difficulty, size) => (difficulty, size),
            _ => panic!("Invalid choice")
        };

        let mut new_scores = self.scores;
        let elem = new_scores
                .iter_mut()
                .find(|c| c.difficulty == difficulty && c.size.is_some() == size.is_some()).unwrap();

        elem.score.last = score;
        elem.score.best = std::cmp::max(elem.score.best, elem.score.last);

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