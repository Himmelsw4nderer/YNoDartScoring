use yew::Reducible;

use crate::models::Throw;
use crate::log_info;

#[derive(Clone, PartialEq)]
pub struct ThrowState {
    pub current_throws: [Option<Throw>; 3],
}

#[derive(Clone)]
pub enum ThrowAction {
    AddThrow(usize, Throw),
    Clear,
}

impl Default for ThrowState {
    fn default() -> Self {
        Self {
            current_throws: [None, None, None],
        }
    }
}


impl Reducible for ThrowState {
    type Action = ThrowAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut new_state = (*self).clone();

        match action {
            ThrowAction::AddThrow(index, throw) => {
                if index < new_state.current_throws.len() {
                    new_state.current_throws[index] = Some(throw);
                    log_info!("Added throw at index {}: {}x{}", index, throw.multiplier, throw.field);
                }
            }
            ThrowAction::Clear => {
                new_state.current_throws = [None, None, None];
                log_info!("Cleared all throws");
            }
        }

        std::rc::Rc::new(new_state)
    }
}

impl ThrowState {
    fn is_valid_finish(&self) -> bool {
        self.current_throws
            .iter()
            .rev()
            .find_map(|throw| throw.as_ref())
            .map(|throw| throw.multiplier == 2)
            .unwrap_or(true)
    }

    pub fn get_score(&self) -> i32 {
        let mut score = 0;
        for current_throw in self.current_throws.iter() {
            if let Some(throw) = current_throw {
                score += throw.get_score();
            }
        }
        score
    }

    pub fn get_clean_score(&self, previous_score: i32) -> Option<i32> {
        let score = self.get_score();
        let current_score = previous_score - score;

        if current_score  < 0 {
            return None;
        }

        if current_score == 1{
            return None;
        }

        if current_score == 0 && !self.is_valid_finish() {
            return None;
        }

        Some(score)
    }

    pub fn get_throw_amount(&self) -> i32 {
        self.current_throws.iter().filter_map(|throw| throw.clone()).count() as i32
    }
}
