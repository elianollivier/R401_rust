use anyhow::Result;
use tokio::io::{self,AsyncBufReadExt,BufReader};
use crate::{configuration::Configuration, domain::{BallotPaper, Candidate, Scoreboard, VotingMachine, Voter, VoteOutcome}};

pub async fn run_app(configuration: Configuration) -> Result<()> {
    let mut voting_machine = create_voting_machine(&configuration);

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
                println!("Liste des votants :");
                for v in &voting_machine.get_voters().0 {
                    println!("- {} ", v.0);
                }
            }

            "scores" => {
                let scoreboard = &voting_machine.get_scoreboard();
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

                let outcome = voting_machine.vote(ballot_paper);

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