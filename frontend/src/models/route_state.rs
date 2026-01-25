use yew::prelude::*;
use std::rc::Rc;

use crate::log_info;

#[derive(Clone, PartialEq)]
pub enum Route {
    Home,
    Game,
    Setup,
    Win,
}

impl Route {
    pub fn name(&self) -> &str {
        match self {
            Route::Home => "Home",
            Route::Game => "Game",
            Route::Setup => "Setup",
            Route::Win => "Win",
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::Home
    }
}

pub enum RouteAction {
    Navigate(Route),
}

#[derive(Clone, PartialEq)]
pub struct RouteState {
    pub current: Route,
}

impl Default for RouteState {
    fn default() -> Self {
        Self {
            current: Route::default(),
        }
    }
}

impl Reducible for RouteState {
    type Action = RouteAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {

        match action {
            RouteAction::Navigate(route) => {
                log_info!("Navigating to: {}", route.name());
                Rc::new(Self { current: route })
            },
        }
    }
}
