use crate::domain::{BallotPaper, VoteOutcome, VotingMachine, Candidate, Voter};
use crate::storage::Storage;
use serde::Deserialize;

#[dervice(Deserialize)]
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
        

        BallotPaper {voter,candidate}
    }
}

pub struct VotingController<Store> {
    store: Store,
}


impl<Store: Storage> VotingController<Store> {
    pub fn new(store: Store) -> Self {
        VotingController {store}
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