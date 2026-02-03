use crate::models::Leg;

#[derive(Clone, PartialEq)]
pub struct Set {
    pub legs: Vec<Leg>,
    pub winner: Option<usize>,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            legs: [Leg::default()].to_vec(),
            winner: None,
        }
    }
}
