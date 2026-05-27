use yew::prelude::*;
use crate::components::{StartingScoreSelector, FirstToLegsSelector, FirstToSetsSelector, PlayerSetup};
use crate::hooks::use_navigator;
use crate::models::{GameAction, Route, GameState, SetupState};
use crate::log_error;


#[function_component(SetupView)]
pub fn setup_view() -> Html {
    let navigator = use_navigator();
    let navigator_start = use_navigator();

    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("GameState context not found - cannot render GameView");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render PlayerSetup");
        return html! { <div>{"Error: Context not available"}</div> };
    };
    let on_start = {
        let game_state = game_state.clone();
        let setup_state = setup_state.clone();
        let navigator_start = navigator_start;
        Callback::from(move |e: MouseEvent| {
            game_state.dispatch(GameAction::ConsumeSetupState((*setup_state).clone()));
            navigator_start(Route::Game).emit(e);
        })
    };

    html! {
        <div class="flex-1 flex flex-col items-center justify-start p-4 w-full max-w-lg mx-auto overflow-y-auto">

            <StartingScoreSelector />

            <FirstToLegsSelector />

            <FirstToSetsSelector />

            <PlayerSetup />

            <div class="mt-auto w-full flex gap-4">
                 <button onclick={navigator(Route::Home)} class="flex-1 py-3 bg-brand-bg text-brand-text font-bold rounded border-2 border-brand-primary uppercase tracking-wider">
                    { "Back" }
                </button>
                <button onclick={on_start} class="flex-[2] py-3 bg-brand-secondary text-brand-text font-bold rounded border-2 border-brand-secondary uppercase tracking-wider">
                    { "Start Game" }
                </button>
            </div>
        </div>
    }
}
