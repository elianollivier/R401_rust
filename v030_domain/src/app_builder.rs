use anyhow::Result;
use std::collections::{HashMap, HashSet};
use tokio::io::{self,AsyncBufReadExt,BufReader};
use crate::configuration::Configuration;

pub async fn run_app(configuration: Configuration) -> Result<()> {
    let mut votants: HashSet<String> = HashSet::new();
    let mut scores: HashMap<String, u32> = HashMap::new();

    for candidate in &configuration.candidates {
        scores.insert(candidate.clone(), 0);
    }

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
                for v in &votants {
                    println!("- {}", v);
                }
            }

            "scores" => {
                println!("Scores :");
                for (candidate, nb) in &scores {
                    println!("{} : {}", candidate, nb);
                }
            }

            "voter" => {

                if parts.len() < 2 {
                    println!("Vous devez indiquer un nom de votant. Ex: voter Tux");
                    continue;
                }

                let votant = parts[1].to_string();

                if votants.contains(&votant) {
                    println!("{} a déjà voté !", votant);
                    continue;
                }

                votants.insert(votant.clone());

                if parts.len() < 3 {
                    if let Some(count) = scores.get_mut("blanc") {
                        *count += 1;
                    }
                    println!("{} a voté blanc", votant);
                } else {
                    let candidat = parts[2].to_string();
                    if scores.contains_key(&candidat) {
                        if let Some(count) = scores.get_mut(&candidat) {
                            *count += 1;
                        }
                        println!("{} a voté pour {}", votant, candidat);
                    } else {
                        if let Some(count) = scores.get_mut("nul") {
                            *count += 1;
                        }
                        println!("Vote nul pour {} (candidat inconnu : {})", votant, candidat);
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