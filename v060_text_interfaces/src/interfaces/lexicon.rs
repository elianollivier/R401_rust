#[derive(PartialEq, Eq, Clone)]
pub struct Lexicon {
    pub blank: &'static str,
    pub invalid: &'static str,
    pub voted_for: &'static str,
    pub already_voted: &'static str,
    pub scoreboard_title: &'static str,
    pub voters_list_title: &'static str,
    pub unknown_command: &'static str,
}
