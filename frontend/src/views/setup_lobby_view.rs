use yew::prelude::*;
use crate::components::{StartingScoreSelector, FirstToLegsSelector, FirstToSetsSelector};
use crate::hooks::use_navigator;
use crate::models::{GameAction, Route, GameState, SetupState};
use crate::log_error;


#[function_component(SetupLobbyView)]
pub fn setup_lobby_view() -> Html {
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
        <table class="border-separate border-spacing-1 p-0 w-full">
            <tr>
                <td colspan=1 class="w-1/3 p-0">
                    <button onclick={navigator(Route::Home)} class="w-full h-full bg-brand-primary text-brand-text rounded-tl-2xl flex flex-col">
                        <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                            <table class="w-full">
                                <tr>
                                    <td class="flex-1">
                                        {"Go"}
                                        <br/>
                                        {"Back"}
                                    </td>
                                    <td class="text-right"><i class="ti ti-arrow-back text-6xl"></i></td>
                                </tr>
                            </table>
                        </div>
                        <div class="text-left p-3 text-sm h-16 flex items-center">
                            {""}
                        </div>
                    </button>
                </td>
                <td colspan=1 class="w-2/3 p-0">
                    <button onclick={on_start} class="h-full w-full bg-brand-secondary text-brand-text rounded-tr-2xl flex flex-col hover:opacity-90 transition-opacity">
                        <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                            <table class="w-full">
                                <tr>
                                    <td class="flex-1">
                                        {"Start"}
                                        <br/>
                                        {"Game"}
                                    </td>
                                    <td class="text-right"><i class="ti ti-target text-6xl"></i></td>
                                </tr>
                            </table>
                        </div>
                        <div class="text-left p-3 text-sm h-16 flex items-center">
                            {""}
                        </div>
                    </button>
                </td>
            </tr>
            <tr class="p-0">
                <td colspan=2 class="p-0 m-0"><div class="w-full bg-brand-bg text-brand-text border border-brand-text">
                    <StartingScoreSelector/>
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=2 class="p-0 m-0"><div class="w-full bg-brand-bg text-brand-text border border-brand-text">
                    <FirstToLegsSelector />
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=2 class="p-0 m-0"><div class="w-full bg-brand-bg text-brand-text border border-brand-text">
                    <FirstToSetsSelector />
                </div></td>
            </tr>

            <tr class="p-0">
                <td colspan=2 class="p-0 m-0"><div class="w-full bg-brand-bg text-brand-text border border-brand-text rounded-b-lg">
                </div></td>
            </tr>
        </table>
    }
}
