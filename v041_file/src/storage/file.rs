use async_trait::async_trait;
use std::path::Path;
use std::fs::{self, File};
use std::collections::{BTreeMap, BTreeSet};
use serde::{Serialize, Deserialize};
use crate::domain::{Candidate, Score, Voter, Scoreboard, VotingMachine, AttendenceSheet};
use crate::storage::Storage;
 
#[derive(Serialize,Deserialize)]
struct ScoreboardDao {
    scores: BTreeMap<String,usize>,
    blank_score: usize,
    invalid_score: usize
}

#[derive(Serialize,Deserialize)]
struct VotingMachineDao {
    voters: BTreeSet<String>,
    scoreboard: ScoreboardDao,
}

impl From<Scoreboard> for ScoreboardDao {
    fn from(scoreboard: Scoreboard) -> Self {
        let mut scores = BTreeMap::new();
        for (candidate, score) in scoreboard.scores{
            scores.insert(candidate.0,score.0);
        }
        ScoreboardDao {
            scores,
            blank_score:scoreboard.blank_score.0,
            invalid_score:scoreboard.invalid_score.0,
        }
    }
}

impl From<ScoreboardDao> for Scoreboard {
    fn from(dao: ScoreboardDao) -> Self {
        let mut scores = BTreeMap::new();
        for (candidate, score) in dao.scores{
            scores.insert(Candidate(candidate.to_string()),Score(score));
        }
        Scoreboard {
            scores,
            blank_score: Score(dao.blank_score),
            invalid_score: Score(dao.invalid_score),
        }
    }
}

impl From<VotingMachine> for VotingMachineDao {
    fn from(machine: VotingMachine) -> Self {
        let mut voters = BTreeSet::new();
        for votant in machine.get_voters().0.iter() {
            let voter_str = votant.0.clone(); 
            voters.insert(voter_str);
        };

        let scoreboard_clone = machine.get_scoreboard().clone();
        let scoreboard_dao = ScoreboardDao::from(scoreboard_clone);
        VotingMachineDao { voters, scoreboard:scoreboard_dao }
    }
}

impl From<VotingMachineDao> for VotingMachine {
    fn from(machinedao: VotingMachineDao) -> Self {
        let mut voters = BTreeSet::new();
        for votant in machinedao.voters {
            voters.insert(Voter(votant));
        };
        let scoreboard = Scoreboard::from(machinedao.scoreboard);

        VotingMachine::recover_from(AttendenceSheet(voters), scoreboard)
    }
}














pub struct FileStore {
    filepath: String,
}

impl FileStore {
    const FILEPATH: &str = "machine.json";

    pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self> {
        if !Path::new(filepath).exists() {
            File::create(filepath)?;
        }
        
        let file = FileStore{
            filepath : filepath.to_string(),
        };

        file.put_voting_machine(machine).await?;

        Ok(file)
    }

    pub async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        FileStore::create(machine, Self::FILEPATH).await
    }
}
#[async_trait]
impl Storage for FileStore {
    
    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let content = fs::read_to_string(&self.filepath)?;
        let dao: VotingMachineDao = serde_json::from_str(&content)?;
        Ok(VotingMachine::from(dao))
    }
    async fn put_voting_machine(&self, machine: VotingMachine) -> anyhow::Result<()> {
        let machinedao = VotingMachineDao::from(machine);
        let json = serde_json::to_string(&machinedao)?;
        fs::write(&self.filepath, json)?;
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use crate::storage::file::FileStore;
    use super::{Candidate, Scoreboard, VotingMachine};
    use crate::storage::Storage;
    use std::fs;

    #[tokio::test]
    async fn test_vote_accepted() {
        let _ = fs::remove_file(FileStore::FILEPATH);
        let candidates = vec![
            Candidate("Trump".to_string()),Candidate("Macron".to_string())
        ];
        let scoreboard = Scoreboard::new(candidates);
        let machine = VotingMachine::new(scoreboard);
        let file_store = FileStore::new(machine.clone()).await.unwrap();
        let result = file_store.get_voting_machine().await.unwrap();

        assert_eq!(
            machine,
            result
        );
        let _ = fs::remove_file(FileStore::FILEPATH);
    }
}