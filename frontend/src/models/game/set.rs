use crate::models::Leg;
use crate::models::Throw;

#[derive(Clone, PartialEq)]
pub struct Set {
    pub legs: Vec<Leg>,
    pub is_won: bool,
    pub set_average: f32,
    pub first_to_legs: i32,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            legs: [Leg::default()].to_vec(),
            is_won: false,
            set_average: 0.0,
            first_to_legs: 3,
        }
    }
}

impl Set {
    fn calculate_set_average(&mut self) {
        let mut total_score = 0.0;
        let mut total_throws = 0;
        for leg in self.legs.iter() {
            total_score += leg.starting_score as f32 - leg.score as f32;
            total_throws += leg.darts_thrown;
        }

        let total_visits = total_throws as f32 / 3.0;
        let set_average = total_score / total_visits as f32;
        if set_average.is_nan() || set_average.is_infinite(){
            self.set_average = 0.0;
            return;
        }
        self.set_average = set_average;
    }

    fn check_win(&mut self){
        let mut legs_won = 0;
        for leg in self.legs.iter() {
            if leg.is_won{
                legs_won += 1;
            }
        }
        if legs_won >= self.first_to_legs{
            self.is_won = true;
        }
    }

    pub fn change_throw(&mut self, leg_index: usize, visit_index: usize, throw_index: usize, throw: Throw) -> Result<(), &'static str>{
        if leg_index >= self.legs.len() {
            return Err("Leg index out of bounds");
        }

        let result = self.legs[leg_index].change_throw(visit_index, throw_index, throw);
        if result.is_ok() {
            self.calculate_set_average();
            self.check_win();
        }

        result
    }
}
