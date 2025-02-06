use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Voter(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Score(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Debug, Clone, PartialEq)]
pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(Debug,PartialEq, Clone)]
pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VotingMachine {
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn new(scoreboard:Scoreboard) -> Self {
        VotingMachine {
            voters : AttendenceSheet(Set::new()),
            scoreboard,
        }
    }

    pub fn vote(&mut self, ballot_paper:BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        } 
        self.voters.0.insert(ballot_paper.voter.clone());
        
        match ballot_paper.candidate {
            None => {
                self.scoreboard.blank_score.0 += 1;
                return VoteOutcome::BlankVote(ballot_paper.voter)
            }
            Some(candidate) => {
                if let Some(score) = self.scoreboard.scores.get_mut(&candidate) {
                    score.0 += 1;
                    VoteOutcome::AcceptedVote(ballot_paper.voter,candidate)
                 } else {
                    self.scoreboard.invalid_score.0 += 1;
                    return VoteOutcome::InvalidVote(ballot_paper.voter)
                 }

            }

        }


        
    }

    pub fn recover_from(voters: crate::domain::AttendenceSheet, scoreboard: Scoreboard) -> Self {
        Self { voters, scoreboard }
    }
    
    pub fn get_scoreboard(&self) -> &Scoreboard {
        return &self.scoreboard;
    }

    pub fn get_voters(&self) -> &AttendenceSheet {
        return &self.voters;
    }

}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        for candidate in candidates {
            scores.insert(candidate,Score(0));
        }
        Scoreboard {
            scores,
            blank_score: Score(0),
            invalid_score : Score(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BallotPaper, Candidate, Scoreboard, VotingMachine, Voter, VoteOutcome};


    fn setup_machine() -> VotingMachine {
        let candidates = vec![
            Candidate("Trump".to_string()),Candidate("Macron".to_string())
        ];
        let scoreboard = Scoreboard::new(candidates);
        VotingMachine::new(scoreboard)
    }

    #[test]
    fn test_vote_accepted() {
        let mut machine = setup_machine();
        
        let name_vote = Voter("Elian".to_string());
        let ballot_paper = BallotPaper {
            voter: name_vote.clone(),
            candidate: Some(Candidate("Trump".to_string())),
        };
        
        let outcome = machine.vote(ballot_paper);

        assert_eq!(
            outcome,
            VoteOutcome::AcceptedVote(name_vote.clone(), Candidate("Trump".to_string()))
        );
    }

    #[test]
    fn test_vote_nul() {
        let mut machine = setup_machine();
        
        let name_vote = Voter("Elian".to_string());
        let ballot_paper = BallotPaper {
            voter: name_vote.clone(),
            candidate: Some(Candidate("feur".to_string())),
        };
        
        let outcome = machine.vote(ballot_paper);

        assert_eq!(
            outcome,
            VoteOutcome::InvalidVote(name_vote.clone())
        );
    }

    #[test]
    fn test_vote_blanc() {
        let mut machine = setup_machine();
        
        let name_vote = Voter("Elian".to_string());
        let ballot_paper = BallotPaper {
            voter: name_vote.clone(),
            candidate: None,
        };
        
        let outcome = machine.vote(ballot_paper);

        assert_eq!(
            outcome,
            VoteOutcome::BlankVote(name_vote.clone())
        );
    }

    #[test]
    fn test_deja_voter() {
        let mut machine = setup_machine();
        
        let name_vote = Voter("Elian".to_string());
        let ballot_paper = BallotPaper {
            voter: name_vote.clone(),
            candidate: Some(Candidate("Trump".to_string())),
        };
        let ballot_paper2 = BallotPaper {
            voter: name_vote.clone(),
            candidate: Some(Candidate("Macron".to_string())),
        };
        
        machine.vote(ballot_paper);
        let outcome = machine.vote(ballot_paper2);

        assert_eq!(
            outcome,
            VoteOutcome::HasAlreadyVoted(name_vote.clone())
        );
    }
}
