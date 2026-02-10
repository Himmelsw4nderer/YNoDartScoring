use yew::prelude::*;
use crate::models::{LegState, LegAction};
use crate::log_error;

#[function_component(StartingScoreSelector)]
pub fn starting_score_selector() -> Html {
    let Some(leg_state) = use_context::<UseReducerHandle<LegState>>() else {
        log_error!("LegState context not found - cannot render StartingScoreSelector");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let starting_scores = vec![301, 501, 701, 901];

    let current_score = leg_state.player_states
        .first()
        .map(|p| p.starting_score)
        .unwrap_or(501);

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
                            let leg_state = leg_state.clone();
                            Callback::from(move |_| {
                                leg_state.dispatch(LegAction::SetStartingScore(starting_score));
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
