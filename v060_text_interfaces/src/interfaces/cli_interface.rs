use anyhow::Result;
use crate::domain::{VoteOutcome, Scoreboard, AttendenceSheet};
use crate::storage::Storage;
use crate::storage::use_cases::{VoteForm, VotingController};
use crate::interfaces::lexicon::Lexicon;


pub async fn handle_line(line: &str,controller: &mut VotingController, lex:&Lexicon) -> Result<String> {
    let command = line.trim();
    if command.is_empty() {
        return Ok("Commande vide ! (voter, votants, scores)".to_string())
    }

    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts[0] {
        "votants" => {
            let machine = controller.get_voting_machine().await?;
            let attendence_sheet = machine.get_voters();
            Ok(show_attendence_sheet(attendence_sheet,lex))
        }
        "scores" => {
            let machine = controller.get_voting_machine().await?;
            let scoreboard = machine.get_scoreboard();
            Ok(show_scoreboard(scoreboard,lex))
        }
        "voter" => {
            if parts.len() < 2 {
                return Ok("Vous devez indiquer un nom de votant. Ex : voter Tux".to_string())
            }
            let votant_name = parts[1].to_string();
            let candidate_name = if parts.len() < 3 {
                "".to_string()
            } else {
                parts[2].to_string()
            };
            let vote_form = VoteForm {
                voter : votant_name,
                candidate : candidate_name,
            };
            let outcome =controller.vote(vote_form).await?;
            Ok(show_vote_outcome(outcome,lex))
        }
        _ => {
            Ok(format!("Commande inconnue : {}", line))
        }

    }
}

fn show_vote_outcome(outcome: VoteOutcome,lex: &Lexicon) -> String {
    match outcome {
        VoteOutcome::BlankVote(v) => format!("{} {}", v.0,lex.blank),
        VoteOutcome::AcceptedVote(v, c) => format!("{} {} {}", v.0,lex.voted_for, c.0),
        VoteOutcome::InvalidVote(v) => format!("{} {}", v.0,lex.invalid),
        VoteOutcome::HasAlreadyVoted(v) => format!("{} {}", v.0,lex.already_voted),
    }
}

fn show_scoreboard(scoreboard: &Scoreboard,lex: &Lexicon) -> String {
    let mut lines = vec![lex.scoreboard_title.to_string()];
    for (candidate, score) in &scoreboard.scores {
        lines.push(format!("{} : {}", candidate.0, score.0));
    }
    lines.push(format!("{} : {}",lex.blank, scoreboard.blank_score.0));
    lines.push(format!("{} : {}",lex.invalid, scoreboard.invalid_score.0));
    lines.join("\n")
}

fn show_attendence_sheet(attendence_sheet: &AttendenceSheet,lex: &Lexicon) -> String {
    let mut lines = vec![lex.voters_list_title.to_string()];
    for v in &attendence_sheet.0 {
        lines.push(format!("- {}", v.0));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{VotingMachine, Scoreboard, Candidate};
    use crate::storage::memory::MemoryStore;
    use crate::storage::use_cases::VotingController;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use crate::interfaces::lexicons::french::french_lexicon;

    #[test]
    fn test_handle_line_empty() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let machine = VotingMachine::new(Scoreboard::new(vec![]));
            let store = Arc::new(MemoryStore::new(machine));
            let mut controller = VotingController::new(store);

            let lex = french_lexicon();
            let result = handle_line("", &mut controller, &lex).await.unwrap();

            assert_eq!(result,"Commande vide ! (voter, votants, scores)".to_string());
        });
    }
    #[test]
    fn test_handle_line_unknown_command() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let machine = VotingMachine::new(Scoreboard::new(vec![]));
            let store = Arc::new(MemoryStore::new(machine));
            let mut controller = VotingController::new(store);

            let lex = french_lexicon();

            let output = handle_line("toto", &mut controller, &lex).await.unwrap();
            assert!(output.contains("Commande inconnue"));
            assert!(output.contains("toto"));
        });
    }

}
