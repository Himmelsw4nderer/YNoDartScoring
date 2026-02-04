use crate::models::{Throw, GamePlayer, Visit};
use crate::{log_info, log_error};
use yew::prelude::*;
use std::rc::Rc;

pub enum GameAction {
    ChangeThrow(Throw, Option<usize>, Option<usize>, Option<usize>, Option<usize>, Option<usize>),
    NextPlayer,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub players: Vec<GamePlayer>,
    pub starting_player: usize,
    pub current_player: usize,
    pub current_set: usize,
    pub current_leg: usize,
    pub current_visit: usize,
    pub current_throw: usize,
    pub winner: Option<usize>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            players: [GamePlayer::default(), GamePlayer::default()].to_vec(),
            starting_player: 0,
            current_player: 0,
            current_set: 0,
            current_leg: 0,
            current_visit: 0,
            current_throw: 0,
            winner: None,
        }
    }
}


impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            GameAction::ChangeThrow(throw, player_index, set_index, leg_index, visit_index, throw_index) => {
                let player_index = player_index.unwrap_or(self.current_player);
                let set_index = set_index.unwrap_or(self.current_set);
                let leg_index = leg_index.unwrap_or(self.current_leg);
                let visit_index = visit_index.unwrap_or(self.current_visit);
                let throw_index = throw_index.unwrap_or(self.current_throw);
                let mut new_state = (*self).clone();

                if player_index >= new_state.players.len() {
                    log_error!("Player index out of range");
                    return self;
                }

                let result = (|| -> Result<(), &'static str> {
                    let player = new_state.players.get_mut(player_index).ok_or("Player index out of bounds")?;
                    let set = player.sets.get_mut(set_index).ok_or("Set index out of bounds")?;
                    let leg = set.legs.get_mut(leg_index).ok_or("Leg index out of bounds")?;
                    leg.change_throw(visit_index, throw_index, throw)
                })();

                match result {
                    Ok(_) => {
                        new_state.current_throw += 1;
                        log_info!("Player {} changed throw at set {}, leg {}, visit {}, throw {}: field={}, multiplier={}",
                                 player_index, set_index, leg_index, visit_index, throw_index, throw.field, throw.multiplier);
                        std::rc::Rc::new(new_state)
                    },
                    Err(error_msg) => {
                        log_error!("Failed to update throw: {}", error_msg);
                        self
                    }
                }
            }

            GameAction::NextPlayer => {
                let mut new_state = (*self).clone();
                let result = (|| -> Result<(), &'static str> {
                    let player = new_state.players.get_mut(new_state.current_player).ok_or("Player index out of bounds")?;
                    let set = player.sets.get_mut(new_state.current_set).ok_or("Set index out of bounds")?;
                    let leg = set.legs.get_mut(new_state.current_leg).ok_or("Leg index out of bounds")?;
                    leg.add_visit();
                    Ok(())
                })();

                if let Err(error_msg) = result {
                    log_error!("Failed to get score for player: {}", error_msg);
                }

                new_state.current_player = (new_state.current_player + 1) % new_state.players.len();
                new_state.current_throw = 0;

                if new_state.current_player == 0 {
                    new_state.current_visit += 1;
                }
                log_info!("Next player: {} -> {}", self.current_player, new_state.current_player);
                std::rc::Rc::new(new_state)
            }
        }
    }
}

impl GameState {
    pub fn get_throw(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>, visit_index: Option<usize>, throw_index: Option<usize>) -> Option<&Throw> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);
        let visit_index = visit_index.unwrap_or(self.current_visit);
        let throw_index = throw_index.unwrap_or(self.current_throw);

        self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .get_throw(visit_index, throw_index)
    }

    pub fn get_leg_score(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<i32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .score)
    }

    pub fn get_darts_thrown(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<i32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .darts_thrown
        )
    }

    pub fn get_last_visit_score(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<i32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .get_last_visit_score()
    }

    pub fn is_leg_bust(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<bool> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .is_bust)
    }

    pub fn get_leg_average(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<f32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .leg_average)
    }

    pub fn get_set_average(&self, player_index: Option<usize>, set_index: Option<usize>) -> Option<f32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .get_set_average())
    }

    pub fn get_average(&self, player_index: Option<usize>) -> Option<f32> {
        let player_index = player_index.unwrap_or(self.current_player);

        Some(self.players.get(player_index)?
            .get_average())
    }
    pub fn has_player_started_leg(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<bool> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        let mut started = player_index == self.starting_player;
        if set_index % self.players.len() != player_index {
            started = !started;
        }
        if leg_index % self.players.len() != player_index {
            started = !started;
        }
        Some(started)
    }

    pub fn get_latest_visit(&self, player_index: Option<usize>, set_index: Option<usize>, leg_index: Option<usize>) -> Option<Visit> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);
        let leg_index = leg_index.unwrap_or(self.current_leg);

        self.players.get(player_index)?
            .sets.get(set_index)?
            .legs.get(leg_index)?
            .visits.last()
            .cloned()
    }
}
