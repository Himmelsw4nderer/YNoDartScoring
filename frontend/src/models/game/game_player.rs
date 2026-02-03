use crate::models::Set;

#[derive(Clone, PartialEq)]
pub struct GamePlayer {
    pub name: String,
    pub sets: Vec<Set>
}

impl Default for GamePlayer {
    fn default() -> Self {
        Self {
            name: String::from("Placeholder"),
            sets: [Set::default()].to_vec(),
        }
    }
}
