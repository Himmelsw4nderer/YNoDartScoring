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
                return self
            }
            SetupAction::RemovePlayer() => {
                return self
            }
            SetupAction::RenamePlayer(index, name) => {
                return self
            }
            SetupAction::SetStartingScore(starting_score) => {
                return self
            }
            SetupAction::SetLegGoal(leg_goal) => {
                return self
            }
            SetupAction::SetSetGoal(set_goal) => {
                return self;
            }
        }
    }
}
