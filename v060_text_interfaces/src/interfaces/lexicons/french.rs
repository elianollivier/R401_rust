use crate::interfaces::lexicon::Lexicon;

pub fn french_lexicon() -> Lexicon {
    Lexicon {
        blank: "blanc",
        invalid: "nul",
        voted_for: "a voté pour",
        already_voted: "a déjà voté",
        scoreboard_title: "Scores :",
        voters_list_title: "Liste des votants :",
        unknown_command: "Commande inconnue :",
    }
}
