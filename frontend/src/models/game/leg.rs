use crate::{models::{Visit, Throw}};

#[derive(Clone, PartialEq)]
pub struct Leg {
    pub visits: Vec<Visit>,
    pub winner: Option<usize>,
    pub starting_score: i32,
    pub score: i32,
}

impl Default for Leg {
    fn default() -> Self {
        let starting_score = 501;
        Self {
            visits: [Visit::default()].to_vec(),
            winner: None,
            starting_score: starting_score,
            score: starting_score,
        }
    }
}

impl Leg {
    fn get_visit_mut(&mut self, index: usize) -> Result<&mut Visit, &'static str> {
        self.visits.get_mut(index).ok_or("Visit index out of bounds")
    }

    fn calculate_score(&self) -> i32 {
        let mut score = self.starting_score;
        for visit in &self.visits {
            score -= visit.get_clean_score(score).unwrap_or(0);
        }
        score
    }

    pub fn calculate_average(&self, current_visit: &Option<Visit>) -> f32 {
        let mut scored_points = self.starting_score - self.score;
        let mut currend_throw_amount = 0;
        if let Some(current_visit) = current_visit {
            let current_throw_score = current_visit.get_clean_score(self.score);
            scored_points += current_throw_score.unwrap_or(0);
            currend_throw_amount = current_visit.get_darts_thrown();
        }
        let total_darts = self.visits.len() as f32 + (currend_throw_amount as f32 * (1.0/3.0));
        let average = scored_points as f32 / total_darts as f32;
        if average.is_nan() {
            return 0.0;
        }
        average
    }

    pub fn change_throw(&mut self, visit_index: usize, throw_index: usize, throw: Throw) -> Result<(), &'static str>{
        self.get_visit_mut(visit_index)?.change_throw(throw_index, throw)?;
        self.score = self.calculate_score();
        Ok(())
    }


    pub fn get_throw(&self, visit_index: usize, throw_index: usize) -> Option<&Throw> {
        if visit_index >= self.visits.len() {
            return None;
        }
        self.visits[visit_index].get_throw(throw_index)
    }

    pub fn add_visit(&mut self) {
        self.visits.push(Visit::default());
    }

    pub fn get_darts_thrown(&self) -> i32 {
        let mut throw_amount = (self.visits.len() as i32 - 1) * 3;
        if let Some(last_visit) = self.visits.last() {
            throw_amount += last_visit.get_darts_thrown();
        }
        throw_amount
    }

    pub fn get_last_visit_score(&self) -> Option<i32> {
        if let Some(second_to_last_visit) = self.visits.iter().rev().nth(1) {
            return Some(second_to_last_visit.get_score());
        }
        None
    }
}
