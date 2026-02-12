use yew::prelude::*;
use crate::log_error;
use crate::models::{SetupState, SetupAction};


#[function_component(StartingScoreSelector)]
pub fn starting_score_selector() -> Html {
    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render StartingScoreSelector");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let starting_scores = vec![301, 501, 701, 901];

    let current_score = setup_state.starting_score;

    html! {
        <div class="w-full mb-8">
            <h2 class="text-2xl text-brand-text font-bold mb-4 text-center">{ "Starting Score" }</h2>
            <div class="grid grid-cols-4 gap-2">
                {
                    starting_scores.into_iter().map(|starting_score| {
                        let is_selected = starting_score == current_score;
                        let bg_class = if is_selected { "bg-brand-primary" } else { "bg-brand-bg" };
                        let border_class = if is_selected { "border-brand-primary" } else { "border-brand-text" };

                        let onclick = {
                            let setup_state = setup_state.clone();
                            Callback::from(move |_| {
                                setup_state.dispatch(SetupAction::SetStartingScore(starting_score));
                            })
                        };

                        html! {
                            <button {onclick} class={format!("py-3 rounded text-xl font-bold border-2 {} {} text-brand-text transition-colors", bg_class, border_class)}>
                                { starting_score }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
