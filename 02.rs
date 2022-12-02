use std::fs::read_to_string;

use Choice::*;
use Outcome::*;

#[derive(Debug)]
enum Outcome {
    WIN = 0,
    LOSE = 1,
    DRAW = 2
}


#[derive(Debug)]
enum Choice {
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            WIN => 6,
            LOSE => 0,
            DRAW => 3,
        }
    }

    fn state(&self) -> usize{ 
        match &self {
            WIN => 0,
            LOSE => 1,
            DRAW => 2,
        }
    }

    pub fn from_str(ch: &str) -> Outcome {
        match ch {
            "X" => LOSE,
            "Y" => DRAW,
            "Z" => WIN,
            _ => unreachable!()
        }
    }
}

impl Choice {
    pub fn from_str(ch: &str) -> Choice {
        // part one
        // match ch {
        //     "A" | "X" => ROCK,
        //     "B" | "Y" => PAPER,
        //     "C" | "Z" => SCISSORS,
        //     _ => unreachable!()
        // }

        match ch {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            a => unreachable!("unreachable char: {}", a)
        }
    }

    fn points(&self) -> usize {
        match &self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3
        }
    }

    fn state(self) -> usize{ 
        match self {
            ROCK => 0,
            PAPER => 1,
            SCISSORS => 2,
        }
    }
}

mod part_one {
    use super::*;
    fn calc_round(a: Choice, b: Choice) -> usize {
        let outcome = match b {
            ROCK => match a {
                ROCK => DRAW,
                PAPER => LOSE,
                SCISSORS => WIN,
            },
            PAPER => match a {
                ROCK => WIN,
                PAPER => DRAW,
                SCISSORS => LOSE
            },
            SCISSORS => match a {
                ROCK => LOSE,
                PAPER => WIN,
                SCISSORS => DRAW
            }
        };


        let choice_pts = b.points();
        let outcome_pts = outcome.points();

        choice_pts + outcome_pts
    }

    pub fn calc_line(line: &str) -> usize {
        if let Some((a, b)) = line.split_once(' ') {
            let (a, b) = (Choice::from_str(a), Choice::from_str(b));
            return calc_round(a, b);
        };
        0
    }

    pub fn solve () {
        let input = read_to_string("./inputs/02.in").unwrap();
        let result: usize = input.lines().map(|line| part_one::calc_line(line)).sum();
        println!("{result}");
    }
}

mod part_two {

    use super::*;
    ///          |WIN     |LOSE    |DRAW
    /// ROCK     |PAPER   |SCISSORS|ROCK
    /// PAPER    |SCISSORS|ROCK    |PAPER
    /// SCISSORS |ROCK    |PAPER   |SCISSORS


    const FSM: [[Choice; 3]; 3] = [
        [PAPER, SCISSORS, ROCK],
        [SCISSORS, ROCK, PAPER],
        [ROCK, PAPER, SCISSORS]
    ];

    fn calc_round(enemy: Choice, outcome: Outcome) -> usize {
        let strat_move = &FSM[enemy.state()][outcome.state()];

        outcome.points() + strat_move.points()
    }
    fn calc_line(line: &str) -> usize {
        if let Some((a, b)) = line.split_once(' ') {
            let (enemy, outcome) = (Choice::from_str(a), Outcome::from_str(b));
            return calc_round(enemy, outcome);
        }
        0
    }
    
    pub fn solve() {
        let input = read_to_string("./inputs/02.in").unwrap();
        let result: usize = input.lines().map(|line| part_two::calc_line(line)).sum();
        println!("{result}");
    }
}


fn main() {
    part_two::solve();
}
