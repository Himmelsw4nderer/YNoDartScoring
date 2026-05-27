use yew::prelude::*;
use crate::checkout::recommend_throws;
use crate::models::{GameState};
use crate::log_error;


#[derive(Properties, PartialEq)]
pub struct ScoreCardProps {
    pub player_index: usize,
}

#[function_component(ScoreCard)]
pub fn score_card(props: &ScoreCardProps) -> Html {

    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("LegState context not found - cannot render ScoreCard");
        return html! { <div>{"Error: Context not available"}</div> };
    };
    let player = &game_state.players[props.player_index];
    let is_turn = game_state.current_player == props.player_index;

    let score = game_state.get_leg_score(Some(props.player_index), None, None).unwrap_or(0);
    let is_bust = game_state.is_leg_bust(Some(props.player_index), None, None).unwrap_or(false);

    let latest_visit = game_state.get_latest_visit(Some(props.player_index), None, None);

    let legs_won = game_state.get_legs_won(Some(props.player_index), None).unwrap_or(0);
    let sets_won = game_state.get_sets_won(Some(props.player_index)).unwrap_or(0);

    let visit_recommendation = if is_bust {
        latest_visit.map(|v| v.throws).unwrap_or_default()
    } else {
        recommend_throws(score, latest_visit)
    };

    let started = game_state.has_player_started_leg(Some(props.player_index), None, None).unwrap_or(false);

    html! {
        <div class={format!("border-y-4 p-3 w-1/2 {}",
            if is_turn {
                "border-brand-primary"
            } else {
                "border-brand-text"
            }
        )}>
        <div class="flex gap-2">
            <div class="text-brand-text text-xl">{&player.name}</div>
            {if started {
                html! { <div class="text-brand-text text-xl">{"⊙"}</div> }
            } else {
                html! {}
            }}
        </div>
        <div class="flex gap-4 items-center">
        <div class="text-brand-secondary font-bold text-6xl">
            {score}
        </div>
        <div class="flex gap-2 ml-auto">
        <table class="">
        <tbody>
            <tr>
                <td class="h-6 bg-brand-bg text-brand-text text-center font-bold border-brand-text">
                    {legs_won}
                </td>
            </tr>
            <tr>
                <td class="h-6 bg-brand-bg text-brand-text text-center border-t border-brand-text">
                    {sets_won}
                </td>
            </tr>
        </tbody>
        </table>
        </div>
        </div>
        </div>
    }
}
