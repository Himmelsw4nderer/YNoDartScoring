use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::BeforeUnloadEvent;


mod components;
mod views;
mod utils;
mod checkout;
mod models;
mod logging;
mod hooks;

use models::{GameState, Route, RouteState, SetupState};

use views::{GameView, HomeView, WinView, SetupView, LobbyView};

#[function_component(App)]
fn app() -> Html {
    let route_state = use_reducer(|| RouteState::default());
    let game_state = use_reducer(|| GameState::default());
    let setup_state = use_reducer(|| SetupState::default());

    let is_game_in_progress = game_state.winner.is_none() && route_state.current == Route::Game;

    use_effect_with(is_game_in_progress, |&is_game_in_progress| {
        let listener = EventListener::new(&gloo::utils::window(), "beforeunload", move |e| {
            if is_game_in_progress {
                if let Some(event) = e.dyn_ref::<BeforeUnloadEvent>() {
                    event.prevent_default();
                    event.set_return_value("A game is in progress. Are you sure you want to leave?");
                }
            }
        });
        move || drop(listener)
    });

    html! {
        <ContextProvider<UseReducerHandle<RouteState>> context={route_state.clone()}>
        <ContextProvider<UseReducerHandle<GameState>> context={game_state}>
        <ContextProvider<UseReducerHandle<SetupState>> context={setup_state}>
            <div class="bg-brand-bg h-dvh flex items-center justify-center">
            <div class="bg-brand-bg p-1 max-w-4xl w-full h-dvh flex flex-col">
                <h1 class="text-4xl font-bold text-center p-3">
                    <span class="text-brand-primary">{ "Y" }</span>
                    <span class="text-brand-secondary">{ "No" }</span>
                    <span class="text-brand-primary">{ "Dart" }</span>
                    <span class="text-brand-text">{ "Scoring" }</span>
                </h1>
                <div class="h-2"></div>
                {
                    match route_state.current {
                        Route::Home => html! { <HomeView /> },
                        Route::Setup => html! { <SetupView /> },
                        Route::Game => html! { <GameView /> },
                        Route::Win => html! { <WinView /> },
                        Route::OnlineLobby => html! { <LobbyView /> },
                    }
                }
                </div>
            </div>
        </ContextProvider<UseReducerHandle<SetupState>>>
        </ContextProvider<UseReducerHandle<GameState>>>
        </ContextProvider<UseReducerHandle<RouteState>>>
    }
}

fn main() {
    logging::init();
    yew::Renderer::<App>::new().render();
}
