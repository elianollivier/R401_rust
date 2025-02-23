pub mod memory;
pub mod file;
use async_trait::async_trait;
use crate::domain::VotingMachine;


#[async_trait]
pub trait Storage {
    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
    async fn put_voting_machine(&self, machine: VotingMachine) -> anyhow::Result<()>;
}