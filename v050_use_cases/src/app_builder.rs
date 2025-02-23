use anyhow::Result;
use std::sync::Arc; 
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::configuration::{Configuration, StorageType};
use crate::domain::{BallotPaper, Candidate, Scoreboard, VoteOutcome, Voter, VotingMachine};
use crate::storage::memory::MemoryStore;
use crate::storage::file::FileStore;
use crate::storage::Storage;

//cargo run -- --candidates Tux Fedora Ubuntu --storage memory

//cargo run -- --candidates Tux Fedora Ubuntu --storage file


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

    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    println!("Bienvenue dans la machine de vote électronique !");
    println!("Commandes disponibles : voter <votant> [candidat], votants, scores");

    while let Some(line) = lines.next_line().await? {
        let command = line.trim();

        if command.is_empty() {
            println!("Commande vide ! (voter, votants, scores)");
            continue;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts[0] {
            "votants" => {
                let machine = store.get_voting_machine().await?;
                println!("Liste des votants :");
                for v in &machine.get_voters().0 {
                    println!("- {} ", v.0);
                }
            }

            "scores" => {
                let machine = store.get_voting_machine().await?;
                let scoreboard = &machine.get_scoreboard();
                println!("Scores :");
                for (candidate, nb) in &scoreboard.scores {
                    println!("{} : {}", candidate.0, nb.0.to_string());
                }
                println!("blanc : {}", &scoreboard.blank_score.0);
                println!("nul : {}", &scoreboard.invalid_score.0);
            }

            "voter" => {

                if parts.len() < 2 {
                    println!("Vous devez indiquer un nom de votant. Ex: voter Tux");
                    continue;
                }

                let votant_name = parts[1].to_string();
                let voter = Voter(votant_name);

                let ballot_paper = if parts.len() < 3 {
                    BallotPaper {
                        voter, 
                        candidate: None,
                    }
                } else {
                    let candidate_name = parts[2].to_string();
                    BallotPaper {
                        voter,
                        candidate: Some(Candidate(candidate_name)),
                    }
                };
                let mut machine = store.get_voting_machine().await?;
                let outcome = machine.vote(ballot_paper);
                store.put_voting_machine(machine).await?; 
                match outcome {
                    VoteOutcome::BlankVote(v) => {
                        println!("{} a voté blanc",v.0);
                    }
                    VoteOutcome::AcceptedVote(v,c) => {
                        println!("{} a voté pour {}",v.0,c.0);
                    }
                    VoteOutcome::InvalidVote(v) => {
                        println!("{} a voté nul",v.0);
                    }
                    VoteOutcome::HasAlreadyVoted(v) => {
                        println!("{} a déjà voter",v.0);
                    }
                }
            }
            _ => {
                println!("Commande inconnue : {}", command);
            }
        }
    }

    Ok(())

}

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let mut candidates = Vec::new();
    for candidate in &configuration.candidates {
        candidates.push(Candidate(candidate.clone()));
    }
    let scoreboard = Scoreboard::new(candidates);

    VotingMachine::new(scoreboard)

}