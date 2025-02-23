#[dervice(Deserialize)]
pub struct VoteFrom {
    pub voter: String,
    pub candidate: String,
}