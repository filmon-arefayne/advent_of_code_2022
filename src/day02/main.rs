use std::{fs::File, io::{BufReader, Read}};

enum Move {
    Rock,
    Paper,
    Scissor,
}
enum Outcome {
    Draw,
    Win,
    Lost
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lost => 0
        }
    }
    fn from_string(str: &str) -> Outcome {
        match str {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            &_ => panic!("aiaiia: {}", str),
        }
    }
}
impl Move {

    fn from_string(str: &str) -> Move {
        match str {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissor,
            &_ => panic!("aiaiia: {}", str),
        }
    }
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn outcome(&self, opponent: &Move) -> Outcome {
        match (self, opponent) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Lost,
            (Move::Rock, Move::Scissor) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissor) => Outcome::Lost,
            (Move::Scissor, Move::Rock) => Outcome::Lost,
            (Move::Scissor, Move::Paper) => Outcome::Win,
            (Move::Scissor, Move::Scissor) => Outcome::Draw,
        }
    }

    fn move_from_outcome(opponent_move: &Move, desired_outcome: &Outcome) -> Move {
        match (opponent_move, desired_outcome) {
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Lost) => Move::Scissor,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissor,
            (Move::Paper, Outcome::Lost) => Move::Rock,
            (Move::Scissor, Outcome::Draw) => Move::Scissor,
            (Move::Scissor, Outcome::Win) => Move::Rock,
            (Move::Scissor, Outcome::Lost) => Move::Paper,
        }   
    }
}

fn main() -> std::io::Result<()>{
    let file = File::open("src/day02/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let total_scores = contents.lines().fold(0, |acc, line|{
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        let opponent_move = Move::from_string(splitted[0]);
        let your_move = Move::from_string(splitted[1]);
        let total_score = your_move.score() + your_move.outcome(&opponent_move).score() + acc;

        total_score
    });

    // part 2 
    let total_scores_v2 = contents.lines().fold(0, |acc, line|{
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        let opponent_move = Move::from_string(splitted[0]);
        let your_outcome = Outcome::from_string(splitted[1]);
        let your_move = Move::move_from_outcome(&opponent_move, &your_outcome);
        let total_score = your_move.score() + your_move.outcome(&opponent_move).score() + acc;

        total_score
    });
    println!("{}", total_scores);
    println!("{}", total_scores_v2);
    Ok(())
}