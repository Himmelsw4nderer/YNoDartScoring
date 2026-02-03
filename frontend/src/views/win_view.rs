use yew::prelude::*;
use crate::hooks::use_navigator;
use crate::models::Route;
use crate::models::{GameState};
use crate::log_error;

#[function_component(WinView)]
pub fn win_view() -> Html {
    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("LegState context not found - cannot render PlayerSetup");
        return html! { <div>{"Error: Context not available"}</div> };
    };
    let navigator = use_navigator();

    html! {
        <div class="flex flex-col gap-4 items-center justify-center h-full">
            <h2 class="text-2xl font-bold text-center text-brand-text">
                <span class="text-brand-primary">{ game_state.players[game_state.winner.unwrap()].name.clone() }</span>
                <span class="text-brand-text">{ " Won" }</span>
                <span class="text-brand-secondary">{ "!!!" }</span>
            </h2>
            <div class="mt-auto w-full flex gap-4">
                <button onclick={navigator(Route::Home)} class="flex-1 py-3 bg-brand-bg text-brand-text font-bold rounded border-2 border-brand-primary uppercase tracking-wider">
                    { "Back" }
                </button>
            </div>
        </div>
    }
}
