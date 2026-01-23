#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Throw {
    pub field: i32,
    pub multiplier: i32,
}

impl Throw {
    pub fn get_score(&self) -> i32 {
        self.field * self.multiplier
    }
}
