use crate::interfaces::lexicon::Lexicon;

pub fn english_lexicon() -> Lexicon {
    Lexicon {
        blank: "blank",
        invalid: "invalid",
        voted_for: "voted for",
        already_voted: "has already voted",
        scoreboard_title: "Scores :",
        voters_list_title: "Voters :",
        unknown_command: "Unknown command :",
    }
}
