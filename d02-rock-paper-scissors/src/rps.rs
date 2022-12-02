/// Trait for states that can be numerically scored.
pub trait Scorable {
    fn points(&self) -> u64;
}

/// Results of a rock-paper-scissors game.
pub enum Result {
    Win,
    Draw,
    Lose,
}

/// Signs that can be used in rock-paper-scissors.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Sign {
    Rock,
    Paper,
    Scissors,
}

/// A single round of rock-paper-scissors.
#[derive(Debug)]
pub struct Round {
    you: Sign,
    opponent: Sign,
}

impl Scorable for Result {
    fn points(&self) -> u64 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

impl Sign {
    /// Get the sign that this sign wins against.
    pub fn wins_against(self) -> Sign {
        match self {
            Sign::Rock => Sign::Scissors,
            Sign::Paper => Sign::Rock,
            Sign::Scissors => Sign::Paper,
        }
    }

    /// Get the sign that this sign looses against.
    pub fn looses_against(self) -> Sign {
        match self {
            Sign::Rock => Sign::Paper,
            Sign::Paper => Sign::Scissors,
            Sign::Scissors => Sign::Rock,
        }
    }

    /// Get the result of a rock-paper-scissors game.
    ///
    /// Returns the result of this sign versus the `other` sign in a game of
    /// rock-paper-scissors.
    ///
    /// # Arguments
    ///
    /// * `other` - The opposing sign.
    fn result(&self, other: &Sign) -> Result {
        if self == other {
            Result::Draw
        } else if self.wins_against() == *other {
            Result::Win
        } else {
            Result::Lose
        }
    }
}

impl Scorable for Sign {
    fn points(&self) -> u64 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }
}

impl Round {
    pub fn new(you: Sign, opponent: Sign) -> Self {
        Self { you, opponent }
    }
}

impl Scorable for Round {
    fn points(&self) -> u64 {
        self.you.points() + self.you.result(&self.opponent).points()
    }
}
