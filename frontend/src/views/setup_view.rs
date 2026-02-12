use yew::prelude::*;
use crate::components::{StartingScoreSelector, PlayerSetup};
use crate::hooks::use_navigator;
use crate::models::Route;

#[function_component(SetupView)]
pub fn setup_view() -> Html {
    let navigator = use_navigator();


    html! {
        <div class="flex-1 flex flex-col items-center justify-start p-4 w-full max-w-lg mx-auto overflow-y-auto">

            <StartingScoreSelector />

            <PlayerSetup />

            <div class="mt-auto w-full flex gap-4">
                 <button onclick={navigator(Route::Home)} class="flex-1 py-3 bg-brand-bg text-brand-text font-bold rounded border-2 border-brand-primary uppercase tracking-wider">
                    { "Back" }
                </button>
                <button onclick={navigator(Route::Game)} class="flex-[2] py-3 bg-brand-secondary text-brand-text font-bold rounded border-2 border-brand-secondary uppercase tracking-wider">
                    { "Start Game" }
                </button>
            </div>
        </div>
    }
}
