use crate::models::ThrowState;

#[derive(Clone, PartialEq)]
pub struct PlayerState {
    pub name: String,
    pub score: i32,
    pub starting_score: i32,
    pub throw_states: Vec<ThrowState>,
}

impl Default for PlayerState {
    fn default() -> Self {
        let starting_score = 501;
        Self {
            name: String::from("NoName"),
            score: starting_score,
            starting_score: starting_score,
            throw_states: vec![],
        }
    }
}

impl PlayerState {
    pub fn change_throw(&mut self, index: Option<usize>, throw_state: ThrowState) {
        if index.is_none() {
            self.throw_states.push(throw_state);
            self.score = self.calculate_score();
            return;
        }
        if let Some(index) = index{
            if index >= self.throw_states.len() {
                return;
            }
            self.throw_states[index] = throw_state;
            self.score = self.calculate_score();
        };
    }

    fn calculate_score(&self) -> i32 {
        let mut score = self.starting_score;
        for throw_state in &self.throw_states {
            score -= throw_state.get_clean_score(score).unwrap_or(0);
        }
        score
    }

    pub fn calculate_average(&self, current_throw_state: &Option<ThrowState>) -> f32 {
        let mut scored_points = self.starting_score - self.score;
        let mut currend_throw_amount = 0;
        if let Some(current_throw_state) = current_throw_state {
            let current_throw_score = current_throw_state.get_clean_score(self.score);
            scored_points += current_throw_score.unwrap_or(0);
            currend_throw_amount = current_throw_state.get_throw_amount();
        }
        let total_darts = self.throw_states.len() as f32 + (currend_throw_amount as f32 * (1.0/3.0));
        let average = scored_points as f32 / total_darts as f32;
        if average.is_nan() {
            return 0.0;
        }
        average
    }
}
