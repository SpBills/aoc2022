use std::fs;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum RPC {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Unknown = 1000
}

impl RPC {
    fn new_from_str(i: &str) -> Self {
        let mut i_c = i.chars();
        let i = i_c.next().unwrap();
        match i {
            'A' | 'X' => RPC::Rock,
            'B' | 'Y' => RPC::Paper,
            'C' | 'Z' => RPC::Scissors,
            _ => RPC::Unknown
        }
    }
}

#[derive(Clone, Copy)]
struct Decision(RPC, RPC);

impl Decision {
    fn decision(&self) -> u32 {
        match (self.0, self.1) {
            (RPC::Rock, RPC::Scissors) => 0,
            (RPC::Paper, RPC::Rock) => 0,
            (RPC::Scissors, RPC::Paper) => 0,
            (RPC::Rock, RPC::Paper) => 6,
            (RPC::Paper, RPC::Scissors) => 6,
            (RPC::Scissors, RPC::Rock) => 6,
            (x, y) if x == y => 3,
            _ => 0,
        }
    }

    fn decision_lose(&self) -> (u32, u32) {
        match (self.0, self.1) {
            (RPC::Rock, RPC::Rock) => (0, RPC::Scissors as u32),
            (RPC::Rock, RPC::Paper) => (3, RPC::Rock as u32),
            (RPC::Rock, RPC::Scissors) => (6, RPC::Paper as u32),
            (RPC::Paper, RPC::Rock) => (0, RPC::Rock as u32),
            (RPC::Paper, RPC::Paper) => (3, RPC::Paper as u32),
            (RPC::Paper, RPC::Scissors) => (6, RPC::Scissors as u32),
            (RPC::Scissors, RPC::Rock) => (0, RPC::Paper as u32),
            (RPC::Scissors, RPC::Paper) => (3, RPC::Scissors as u32),
            (RPC::Scissors, RPC::Scissors) => (6, RPC::Rock as u32),
            _ => (0, 0),
        }
    }
}

fn part_1(decisions: Vec<Decision>) -> u32 {
    let mut score: u32 = 0;

    for decision in decisions {
        score += decision.decision();
    }

    return score;
}

fn part_2(decisions: Vec<Decision>) -> u32 {
    let mut score: u32 = 0;

    for decision in decisions {
        let d = decision.decision_lose();
        score += d.0 + d.1;
    }

    return score;
}

fn main() {
    let data = fs::read_to_string("./inp/d2.txt").unwrap();
    let decisions = data
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|a| Decision(RPC::new_from_str(a[0]), RPC::new_from_str(a[1])))
        .collect::<Vec<_>>();
    let score = part_1(decisions.clone());
    println!("{}", score);
    let score = part_2(decisions.clone());
    println!("{}", score);
}
