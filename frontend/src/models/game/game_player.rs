use crate::models::Set;

#[derive(Clone, PartialEq)]
pub struct GamePlayer {
    pub name: String,
    pub sets: Vec<Set>,
}

impl Default for GamePlayer {
    fn default() -> Self {
        Self {
            name: String::from("Placeholder"),
            sets: [Set::default()].to_vec(),
        }
    }
}

impl GamePlayer {
    pub fn get_average(&self) -> f32{
        let mut total_score = 0.0;
        let mut total_throws = 0;
        for set in self.sets.iter() {
            for leg in set.legs.iter() {
                total_score += leg.starting_score as f32 - leg.score as f32;
                total_throws += leg.darts_thrown;
            }
        }

        let total_visits = total_throws as f32 / 3.0;
        let average = total_score / total_visits;
        if average.is_nan() || average.is_infinite(){
            return 0.0;
        }
        average
    }
}
