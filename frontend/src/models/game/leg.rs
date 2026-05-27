use crate::{models::{Visit, Throw}};

#[derive(Clone, PartialEq)]
pub struct Leg {
    pub visits: Vec<Visit>,
    pub winner: Option<usize>,
    pub starting_score: i32,
    pub score: i32,
    pub is_bust: bool,
    pub leg_average: f32,
    pub darts_thrown: i32,
    pub is_won: bool,
}

impl Default for Leg {
    fn default() -> Self {
        Leg::new(501)
    }
}

impl Leg {
    fn get_visit_mut(&mut self, index: usize) -> Result<&mut Visit, &'static str> {
        self.visits.get_mut(index).ok_or("Visit index out of bounds")
    }

    fn calculate_score(&mut self) {
        let mut score = self.starting_score;
        let mut is_bust = false;
        for visit in &self.visits {
            is_bust = false;
            if let Some(visit_score) = visit.get_clean_score(score) {
                score -= visit_score;
            } else {
                is_bust = true;
                break;
            }
        }
        self.score = score;
        self.is_bust = is_bust;
    }

    fn calculate_darts_thrown(&mut self) {
        let mut darts_thrown = (self.visits.len() as i32 - 1) * 3;
        if let Some(last_visit) = self.visits.last() {
            darts_thrown += last_visit.get_darts_thrown();
        }
        self.darts_thrown = darts_thrown;
    }

    fn calculate_leg_average(&mut self) {
        let scored_points = self.starting_score - self.score;
        let visits = self.darts_thrown as f32 / 3.0;
        let mut average = scored_points as f32 / visits;
        if average.is_nan() {
            average = 0.0;
        }
        self.leg_average = average;
    }

    pub fn check_win(&mut self) {
        if self.score == 0 {
            self.is_won = true;
        }
    }

    pub fn change_throw(&mut self, visit_index: usize, throw_index: usize, throw: Throw) -> Result<(), &'static str>{
        self.get_visit_mut(visit_index)?.change_throw(throw_index, throw)?;
        self.calculate_score();
        self.calculate_darts_thrown();
        self.calculate_leg_average();
        self.check_win();
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

    pub fn get_last_visit_score(&self) -> Option<i32> {
        if let Some(second_to_last_visit) = self.visits.iter().rev().nth(1) {
            return Some(second_to_last_visit.get_score());
        }
        None
    }

    pub fn new(starting_score: i32) -> Self {
        Self {
            visits: [Visit::default()].to_vec(),
            winner: None,
            starting_score: starting_score,
            score: starting_score,
            is_bust: false,
            leg_average: 0.0,
            darts_thrown: 0,
            is_won: false,
        }
    }
}
