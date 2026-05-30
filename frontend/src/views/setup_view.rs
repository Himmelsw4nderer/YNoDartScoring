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
        <div class="items-center justify-start p-y-4 w-full max-w-lg mx-auto overflow-y-auto border-brand-text">

            <table class="border-separate border-spacing-1 w-full">
            <tr class="p-0">
                <td colspan=3><div class="border rounded-t-2xl border-brand-text">
                    <StartingScoreSelector/>
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=3><div class="border border-brand-text">
                    <FirstToLegsSelector />
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=3><div class="border border-brand-text">
                    <FirstToSetsSelector />
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=3><div class="border border-brand-text">
                    <PlayerSetup />
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=1>
                        <button onclick={navigator(Route::Home)} class="text-2xl text-left w-full p-3 bg-brand-bg text-brand-text border border-brand-primary bg-brand-primary rounded-bl-2xl">
                            { "Back" }
                        </button>
                    </td>
                    <td colspan=2>
                        <button onclick={on_start} class="text-2xl text-right w-full p-3 bg-brand-secondary text-brand-text border border-brand-secondary rounded-br-2xl">
                            { "Start Game" }
                        </button>
                    </td>
                </tr>
            </table>
        </div>
    }
}
