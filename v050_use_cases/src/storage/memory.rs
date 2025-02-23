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

mod tests {
    #[tokio::test]
    async fn test_pub_and_get() {
        let candidates = vec![
            Candidate("Trump".to_string()),Candidate("Macron".to_string())
        ];
        let scoreboard = Scoreboard::new(candidates);
        let machine = VotingMachine::new(scoreboard);
        let store = MemoryStore::new(machine.clone());
        let _ = store.put_voting_machine(machine.clone()).await;
        let result = store.get_voting_machine().await.unwrap();
        
        assert_eq!(machine,result);
    }

    
}
