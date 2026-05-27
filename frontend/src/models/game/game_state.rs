use crate::models::{Throw, GamePlayer, Visit, SetupState};
use crate::{log_info, log_error};
use yew::prelude::*;
use std::rc::Rc;

pub enum GameAction {
    ChangeThrow(Throw, Option<usize>, Option<usize>, Option<usize>, Option<usize>, Option<usize>),
    NextPlayer,
    ConsumeSetupState(SetupState)
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
        Self::new(vec![GamePlayer::default(), GamePlayer::default()], 0)
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
                    player.change_throw(set_index, leg_index, visit_index, throw_index, throw)
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
                let mut is_leg_won = false;
                let mut is_set_won = false;

                let result = (|| -> Result<(), &'static str> {
                    let player = new_state.players.get_mut(new_state.current_player).ok_or("Player index out of bounds")?;
                    let set = player.sets.get_mut(new_state.current_set).ok_or("Set index out of bounds")?;
                    let leg = set.legs.get_mut(new_state.current_leg).ok_or("Leg index out of bounds")?;

                    if player.is_won {
                        new_state.winner = Some(new_state.current_player);
                    } else if leg.is_won {
                        is_leg_won = true;
                    } else if set.is_won {
                        is_set_won = true;
                    }
                    Ok(())
                })();

                if let Err(error_msg) = result {
                    log_error!("Failed to get for player: {}", error_msg);
                }

                if new_state.winner.is_some() {
                    log_info!("Player won game: {}", self.current_player);
                    return std::rc::Rc::new(new_state);
                }
                if is_set_won {
                    log_info!("Player finised Set: {}", self.current_player);
                    new_state.new_set();
                } else if is_leg_won {
                    log_info!("Player finised leg: {}", self.current_player);
                    new_state.new_leg();
                } else {
                    let old_player = new_state.current_player;
                    new_state.current_player = (new_state.current_player + 1) % new_state.players.len();
                    new_state.current_throw = 0;

                    if new_state.current_player == 0 {
                        new_state.new_visit();
                    }

                    log_info!("Next player: {} -> {}", old_player, new_state.current_player);
                }

                std::rc::Rc::new(new_state)
            }

            GameAction::ConsumeSetupState(setup_state) => {
                let players: Vec<GamePlayer> = setup_state
                    .players
                    .into_iter()
                    .enumerate()
                    .map(|(index, player)| {
                        let name = if player.name.trim().is_empty() {
                            format!("Player {}", index + 1)
                        } else {
                            player.name
                        };
                        GamePlayer::new(name, setup_state.set_goal, setup_state.leg_goal, setup_state.starting_score)
                    })
                    .collect();

                let new_state = GameState::new(players, 0);

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
            .set_average)
    }

    pub fn get_average(&self, player_index: Option<usize>) -> Option<f32> {
        let player_index = player_index.unwrap_or(self.current_player);

        Some(self.players.get(player_index)?
            .average)
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

    pub fn get_legs_won(&self, player_index: Option<usize>, set_index: Option<usize>) -> Option<i32> {
        let player_index = player_index.unwrap_or(self.current_player);
        let set_index = set_index.unwrap_or(self.current_set);

        Some(self.players.get(player_index)?
            .sets.get(set_index)?
            .legs_won)
    }

    pub fn get_sets_won(&self, player_index: Option<usize>) -> Option<i32> {
        let player_index = player_index.unwrap_or(self.current_player);

        Some(self.players.get(player_index)?
            .sets_won)
    }

    fn new_visit(&mut self) {
        self.current_visit += 1;
        self.current_throw = 0;

        for player in self.players.iter_mut() {
            if let Some(set) = player.sets.get_mut(self.current_set) {
                if let Some(leg) = set.legs.get_mut(self.current_leg) {
                    leg.add_visit();
                }
            }
        }
    }

    fn new_leg(&mut self) {
        self.current_leg += 1;
        self.current_visit = 0;
        self.current_throw = 0;

        let player_starts: Vec<bool> = (0..self.players.len())
            .map(|i| self.has_player_started_leg(Some(i), None, None).unwrap_or(false))
            .collect();

        for (player_index, player) in self.players.iter_mut().enumerate() {
            if let Some(set) = player.sets.get_mut(self.current_set) {
                set.legs.push(crate::models::Leg::default());
            }
            if player_starts[player_index] {
                self.current_player = player_index;
            }
        }
    }

    fn new_set(&mut self) {
        self.current_set += 1;
        self.current_leg = 0;
        self.current_visit = 0;
        self.current_throw = 0;

        let player_starts: Vec<bool> = (0..self.players.len())
            .map(|i| self.has_player_started_leg(Some(i), None, None).unwrap_or(false))
            .collect();

        for (player_index, player) in self.players.iter_mut().enumerate() {
            player.sets.push(crate::models::Set::default());
            if player_starts[player_index] {
                self.current_player = player_index;
            }
        }
    }

    pub fn new(mut players: Vec<GamePlayer>, starting_player: usize) -> Self {
        if players.is_empty() {
            players = vec![GamePlayer::default(), GamePlayer::default()];
        }

        let max_start = players.len().saturating_sub(1);
        let starting_player = starting_player.min(max_start);

        Self {
            players,
            starting_player,
            current_player: starting_player,
            current_set: 0,
            current_leg: 0,
            current_visit: 0,
            current_throw: 0,
            winner: None,
        }
    }
}
