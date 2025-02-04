use crate::domain::VotingMachine;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use super::Storage;




pub struct MemoryStore {
    store: Arc<RwLock<VotingMachine>>,
}

impl MemoryStore {
    pub fn new(machine:VotingMachine) -> Self {
        MemoryStore {
            store: Arc::new(RwLock::new(machine)),
        }
    }
}
#[async_trait]
impl Storage for MemoryStore {

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let machine = self.store.read().await;
        Ok(machine.clone())
        
    }

    async fn put_voting_machine(&self, machine: VotingMachine) -> anyhow::Result<()> {
        let mut machine_structure = self.store.write().await;
        *machine_structure = machine;
        Ok(())
    }



}