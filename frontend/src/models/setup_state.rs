use yew::prelude::*;
use std::rc::Rc;

use crate::log_info;

#[derive(Clone, PartialEq)]
pub struct SetupPlayer{
    pub name: String,
    pub playable: bool,
}

impl Default for SetupPlayer {
    fn default() -> Self {
        Self {
            name: String::from(""),
            playable: true,
        }
    }
}

pub enum SetupAction {
    AddPlayer(),
    RemovePlayer(),
    RenamePlayer(usize, String),
    SetStartingScore(i32),
    SetLegGoal(i32),
    SetSetGoal(i32),
}

#[derive(Clone, PartialEq)]
pub struct SetupState {
    pub players: Vec<SetupPlayer>,
    pub starting_score: i32,
    pub leg_goal: i32,
    pub set_goal: i32,
}

impl Default for SetupState {
    fn default() -> Self {
        Self {
            players: [SetupPlayer::default(), SetupPlayer::default()].to_vec(),
            starting_score: 501,
            leg_goal: 3,
            set_goal: 1,
        }
    }
}

impl Reducible for SetupState {
    type Action = SetupAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SetupAction::AddPlayer() => {
                let mut new_state = (*self).clone();
                if new_state.players.len() < 4{
                    log_info!("Adding a new player");
                    new_state.players.push(SetupPlayer::default());
                }
                Rc::new(new_state)
            }
            SetupAction::RemovePlayer() => {
                let mut new_state = (*self).clone();
                if new_state.players.len() > 1 {
                    log_info!("Removing the last player");
                    new_state.players.pop();
                }
                Rc::new(new_state)
            }
            SetupAction::RenamePlayer(index, name) => {
                let mut new_state = (*self).clone();
                log_info!("Renaming player at index {} to {}", index, name);
                if let Some(player) = new_state.players.get_mut(index) {
                    player.name = name;
                }
                Rc::new(new_state)
            }
            SetupAction::SetStartingScore(starting_score) => {
                let mut new_state = (*self).clone();
                log_info!("Setting starting score to {}", starting_score);
                new_state.starting_score = starting_score;
                Rc::new(new_state)
            }
            SetupAction::SetLegGoal(leg_goal) => {
                let mut new_state = (*self).clone();
                log_info!("Setting leg goal to {}", leg_goal);
                new_state.leg_goal = leg_goal;
                Rc::new(new_state)
            }
            SetupAction::SetSetGoal(set_goal) => {
                let mut new_state = (*self).clone();
                log_info!("Setting set goal to {}", set_goal);
                new_state.set_goal = set_goal;
                Rc::new(new_state)
            }
        }
    }
}
