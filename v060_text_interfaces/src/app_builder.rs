use anyhow::Result;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::configuration::{Configuration, Language, StorageType};
use crate::domain::{Candidate, Scoreboard, VoteOutcome, VotingMachine};
use crate::interfaces::lexicons::english::english_lexicon;
use crate::storage::memory::MemoryStore;
use crate::storage::file::FileStore;
use crate::storage::use_cases::{VoteForm, VotingController};
use crate::storage::Storage;

// cargo run -- --candidates Tux Fedora Ubuntu --storage memory --language en
// cargo run -- --candidates Tux Fedora Ubuntu --storage file --language fr

pub async fn run_app(configuration: Configuration) -> Result<()> {
    let voting_machine = create_voting_machine(&configuration);

    let store: Arc<dyn Storage + Send + Sync> = match configuration.storage {
        StorageType::File => {
            Arc::new(FileStore::new(voting_machine).await?) as Arc<dyn Storage + Send + Sync>
        },
        StorageType::Memory => {
            Arc::new(MemoryStore::new(voting_machine)) as Arc<dyn Storage + Send + Sync>
        },
    };

    let mut controller = VotingController::new(store);

    let lex = match configuration.language {
        Language::Fr => french_lexicon(),
        Language::En => english_lexicon(),
    };


    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    println!("Bienvenue dans la machine de vote électronique !");
    println!("Commandes disponibles : voter <votant> [candidat], votants, scores");

    while let Some(line) = lines.next_line().await? {
        let result_text = handle_line(&line, &mut controller,&lex).await?;
        println!("{}",result_text);
    }

    Ok(())
}

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let candidates: Vec<Candidate> = configuration
        .candidates
        .iter()
        .map(|c| Candidate(c.clone()))
        .collect();
    let scoreboard = Scoreboard::new(candidates);
    VotingMachine::new(scoreboard)
}
