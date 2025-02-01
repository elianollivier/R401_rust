use std::collections::BTreeMap as Map:
use std::collections::BTreeSet as Set;

pub struct Voter(pub String);

pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendenceSheet(pub Set<Voter>);

pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank score: Score,
    pub invalid_score: Score,
}
pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}
pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}
pub struct VotingMachine {
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn new() -> Self {
        let mut scores = Map::new<>;
        for candidate in candidates {
            scores.insert(candidate,Score(0))
        }
        VotingMachine {
            voters: Set::new<Voter>,
            blanc_score: Score(0),
            invalid_score : Score(0),
        }
    }
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new<>;
        for candidate in candidates {
            scores.insert(candidate,Score(0))
        }
        Scoreboard {
            scores,
            blanc_score: Score(0),
            invalid_score : Score(0),
        }
    }
}