use crate::domain::{BallotPaper, VoteOutcome, VotingMachine, Candidate, Voter};
use crate::storage::Storage;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)] 
pub struct VoteForm {
    pub voter: String,
    pub candidate: String,
}

impl From<VoteForm> for BallotPaper {
    fn from(form: VoteForm) -> Self {
        let voter = Voter(form.voter);
        let candidate = if form.candidate.trim().is_empty() {
            None
        } else {
            Some(Candidate(form.candidate))
        };
        BallotPaper { voter, candidate }
    }
}

pub struct VotingController {
    store: Arc<dyn Storage + Send + Sync>,
}

impl VotingController {
    pub fn new(store: Arc<dyn Storage + Send + Sync>) -> Self {
        VotingController { store }
    }

    pub async fn vote(&mut self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
        let mut machine = self.store.get_voting_machine().await?;
        let ballot = BallotPaper::from(vote_form);
        let outcome = machine.vote(ballot);
        self.store.put_voting_machine(machine).await?;
        Ok(outcome)
    }

    pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        self.store.get_voting_machine().await
    }
}

#[cfg(test)]
mod tests {
    use super::{VoteForm, VotingController};
    use crate::domain::{Candidate, Scoreboard, VoteOutcome, Voter, VotingMachine};
    use crate::storage::memory::MemoryStore;
    use anyhow::Result;
    use std::sync::Arc;

    fn setup_machine() -> VotingMachine {
        let candidates = vec![
            Candidate("Trump".to_string()),
            Candidate("Macron".to_string()),
        ];
        let scoreboard = Scoreboard::new(candidates);
        VotingMachine::new(scoreboard)
    }

    #[tokio::test]
    async fn test_vote_accepted() -> Result<()> {
        let machine = setup_machine();
        let store = Arc::new(MemoryStore::new(machine));
        let mut controller = VotingController::new(store);

        let vote_form = VoteForm {
            voter: "Elian".to_string(),
            candidate: "Trump".to_string(),
        };
        let outcome = controller.vote(vote_form).await?;
        assert_eq!(
            outcome,
            VoteOutcome::AcceptedVote(Voter("Elian".to_string()), Candidate("Trump".to_string()))
        );
        let machine = controller.get_voting_machine().await?;
        let trump_score = machine
            .get_scoreboard()
            .scores
            .get(&Candidate("Trump".to_string()))
            .unwrap()
            .0;
        assert_eq!(trump_score, 1);
        Ok(())
    }

}
