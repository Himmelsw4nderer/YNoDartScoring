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



impl Set {
    pub fn get_set_average(&self) -> f32 {
        let mut total_score = 0.0;
        let mut total_throws = 0;
        for leg in self.legs.iter() {
            total_score += leg.starting_score as f32 - leg.score as f32;
            total_throws += leg.darts_thrown;
        }

        let total_visits = total_throws as f32 / 3.0;
        let set_average = total_score / total_visits as f32;
        if set_average.is_nan() || set_average.is_infinite(){
            return 0.0;
        }
        set_average
    }
}
