use yew::prelude::*;
use std::rc::Rc;
use crate::models::{PlayerState, ThrowState};
use crate::{log_info};

pub enum LegAction {
    ChangeThrow(usize, Option<usize>, ThrowState),
    NextPlayer,
    SetStartingScore(i32),
    AddPlayer,
    RemovePlayer(usize),
    SetPlayerName(usize, String),
}

#[derive(Clone, PartialEq)]
pub struct LegState {
    pub player_states: Vec<PlayerState>,
    pub current_player: usize,
    pub starting_player: usize,
    pub winner: Option<usize>,
}

impl Default for LegState {
    fn default() -> Self {
        Self {
            player_states: [PlayerState::default(), PlayerState::default()].to_vec(),
            current_player: 0,
            starting_player: 0,
            winner: None,
        }
    }
}


impl Reducible for LegState {
    type Action = LegAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            LegAction::ChangeThrow(player_index, index, throw_state) => {
                log_info!("Player {} changed throw at index {:?}: {:?}", player_index, index, throw_state.current_throws);
                let mut new_state = (*self).clone();
                new_state.player_states[player_index].change_throw(index, throw_state);
                if new_state.player_states[player_index].score == 0 {
                    new_state.winner = Some(player_index);
                }
                std::rc::Rc::new(new_state)
            }
            LegAction::NextPlayer => {
                let mut new_state = (*self).clone();
                new_state.current_player = (new_state.current_player + 1) % new_state.player_states.len();
                log_info!("Next player: {} -> {}", self.current_player, new_state.current_player);
                std::rc::Rc::new(new_state)
            }
            LegAction::SetStartingScore(starting_score) => {
                log_info!("Setting starting score to {}", starting_score);
                let mut new_state = (*self).clone();
                for player in &mut new_state.player_states {
                    player.set_starting_score(starting_score);
                }
                std::rc::Rc::new(new_state)
            }
            LegAction::SetPlayerName(player_index, name) => {
                log_info!("Setting player {} name to {}", player_index, name);
                let mut new_state = (*self).clone();
                new_state.player_states[player_index].name = name;
                std::rc::Rc::new(new_state)
            }
            LegAction::AddPlayer => {
                log_info!("Adding Player to the game");
                let mut new_state = (*self).clone();
                let starting_score = new_state.player_states.first().map(|p| p.starting_score).unwrap_or(501);
                let mut new_player = PlayerState::default();
                new_player.set_starting_score(starting_score);
                new_state.player_states.push(new_player);
                std::rc::Rc::new(new_state)
            }
            LegAction::RemovePlayer(player_index) => {
                log_info!("Removing Player {} from the game", player_index);
                let mut new_state = (*self).clone();
                new_state.player_states.remove(player_index);
                std::rc::Rc::new(new_state)
            }
        }
    }
}
