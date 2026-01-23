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

use models::LegState;
use models::ThrowState;

use views::{GameView, HomeView};

#[derive(Clone, PartialEq)]
enum Route {
    Home,
    Game,
}

#[function_component(App)]
fn app() -> Html {
    let current_route = use_state(|| Route::Home);
    let leg_state = use_reducer(|| LegState::default());
    let throw_state = use_reducer(|| ThrowState::default());

    let is_game_in_progress = leg_state.winner.is_none() && *current_route == Route::Game;

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
        <ContextProvider<UseReducerHandle<LegState>> context={leg_state}>
        <ContextProvider<UseReducerHandle<ThrowState>> context={throw_state}>
            <div class="bg-brand-bg h-dvh flex items-center justify-center">
            <div class="bg-brand-bg p-1 max-w-4xl w-full h-dvh flex flex-col">
                <h1 class="text-4xl font-bold text-center mb-8 p-3">
                    <span class="text-brand-primary">{ "Y" }</span>
                    <span class="text-brand-secondary">{ "No" }</span>
                    <span class="text-brand-primary">{ "Dart" }</span>
                    <span class="text-brand-text">{ "Scoring" }</span>
                </h1>
                <div class="h-2"></div>
                {
                    match *current_route {
                        Route::Home => html! { <HomeView /> },
                        Route::Game => html! { <GameView /> },
                    }
                }
                </div>
            </div>
        </ContextProvider<UseReducerHandle<ThrowState>>>
        </ContextProvider<UseReducerHandle<LegState>>>
    }
}

fn main() {
    logging::init();
    yew::Renderer::<App>::new().render();
}
