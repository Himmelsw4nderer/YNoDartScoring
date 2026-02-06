use crate::models::Set;
use crate::models::Throw;

#[derive(Clone, PartialEq)]
pub struct GamePlayer {
    pub name: String,
    pub sets: Vec<Set>,
    pub average: f32,
}

impl Default for GamePlayer {
    fn default() -> Self {
        Self {
            name: String::from("Placeholder"),
            sets: [Set::default()].to_vec(),
            average: 0.0,
        }
    }
}

impl GamePlayer {
    pub fn calculate_average(&mut self){
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
            self.average = 0.0;
            return;
        }
        self.average = average;
    }

    pub fn change_throw(&mut self, set_index: usize, leg_index: usize, visit_index: usize, throw_index: usize, throw: Throw) -> Result<(), &'static str>{
        if set_index >= self.sets.len() {
            return Err("Leg index out of bounds");
        }

        let result = self.sets[set_index].change_throw(leg_index, visit_index, throw_index, throw);
        if result.is_ok() {
            self.calculate_average();
        }
        result
    }
}
