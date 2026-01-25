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

use models::{LegState, ThrowState, Route, RouteState};

use views::{GameView, HomeView, SetupView, WinView};

#[function_component(App)]
fn app() -> Html {
    let route_state = use_reducer(|| RouteState::default());
    let leg_state = use_reducer(|| LegState::default());
    let throw_state = use_reducer(|| ThrowState::default());

    let is_game_in_progress = leg_state.winner.is_none() && route_state.current == Route::Game;

    {
        let is_game_in_progress = is_game_in_progress.clone();
        use_effect(move || {
            let listener = EventListener::new(&gloo::utils::window(), "beforeunload", move |e| {
                if is_game_in_progress {
                    if let Some(event) = e.dyn_ref::<BeforeUnloadEvent>() {
                        event.prevent_default();
                        event.set_return_value("A game is in progress. Are you sure you want to leave?");
                    }
                }
            });
            || drop(listener)
        });
    }

    html! {
        <ContextProvider<UseReducerHandle<RouteState>> context={route_state.clone()}>
        <ContextProvider<UseReducerHandle<LegState>> context={leg_state}>
        <ContextProvider<UseReducerHandle<ThrowState>> context={throw_state}>
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
                        Route::Game => html! { <GameView /> },
                        Route::Setup => html! { <SetupView /> },
                        Route::Win => html! { <WinView /> },
                    }
                }
                </div>
            </div>
        </ContextProvider<UseReducerHandle<ThrowState>>>
        </ContextProvider<UseReducerHandle<LegState>>>
        </ContextProvider<UseReducerHandle<RouteState>>>
    }
}

fn main() {
    logging::init();
    yew::Renderer::<App>::new().render();
}
