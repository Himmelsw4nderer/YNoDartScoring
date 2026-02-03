use crate::{models::Throw};

#[derive(Clone, PartialEq)]
pub struct Visit {
    pub throws: [Option<Throw>; 3],
}


impl Default for Visit {
    fn default() -> Self {
        Self {
            throws: [None, None, None],
        }
    }
}


impl Visit {
    fn is_valid_finish(&self) -> bool {
        self.throws
            .iter()
            .rev()
            .find_map(|throw| throw.as_ref())
            .map(|throw| throw.multiplier == 2)
            .unwrap_or(true)
    }

    pub fn get_score(&self) -> i32 {
        let mut score = 0;
        for current_throw in self.throws.iter() {
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

    pub fn get_darts_thrown(&self) -> i32 {
        self.throws.iter().filter_map(|throw| throw.clone()).count() as i32
    }

    pub fn change_throw(&mut self, throw_index: usize, throw: Throw) -> Result<(), &'static str> {
        if throw_index >= self.throws.len() {
            return Err("Throw index out of bounds");
        }
        self.throws[throw_index] = Some(throw);
        Ok(())
    }

    pub fn get_throw(&self, throw_index: usize) -> Option<&Throw> {
        if throw_index >= self.throws.len() {
            return None;
        }
        self.throws[throw_index].as_ref()
    }
}
