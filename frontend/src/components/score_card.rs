use yew::prelude::*;
use crate::utils::translate_multiplier_to_char;
use crate::checkout::recommend_throws;
use crate::models::{GameState, Throw};
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

    let visit_recommendation = if is_bust {
        latest_visit.map(|v| v.throws).unwrap_or_default()
    } else {
        recommend_throws(score, latest_visit)
    };


    let leg_average = game_state.get_leg_average(Some(props.player_index), None, None).unwrap_or(0.0);
    let set_average = game_state.get_set_average(Some(props.player_index), None).unwrap_or(0.0);
    let average = game_state.get_average(Some(props.player_index)).unwrap_or(0.0);


    let started = game_state.has_player_started_leg(Some(props.player_index), None, None).unwrap_or(false);

    let last_visit_score = game_state.get_last_visit_score(Some(props.player_index), None, None).unwrap_or(0); // TODO implement
    let darts_thrown = game_state.get_darts_thrown(Some(props.player_index), None, None).unwrap_or(0); // TODO implement

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
                    {0}
                </td>
            </tr>
            <tr>
                <td class="h-6 bg-brand-bg text-brand-text text-center border-t border-brand-text">
                    {0}
                </td>
            </tr>
        </tbody>
        </table>
        </div>
        </div>
        <table class="w-full bg-brand-text p-1">
        <tbody>
            <tr>
            <td class={format!("w-1/3 h-8 border-r border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center {}",
                if let Some(throw) = visit_recommendation[0] {
                    if throw.multiplier == 2 {
                        "border-b-brand-secondary"
                } else if throw.multiplier == 3 {
                    "border-b-brand-primary"
                } else {
                    "border-b-brand-text"
                }
                } else {
                    "border-b-brand-text"
                }
            )}>
            <span class="text-sm font-bold text-brand-bg">
                {
                    if let Some(throw) = visit_recommendation[0] {
                        format!("{}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            <td class={format!("w-1/3 h-8 border-x border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center {}",
                if let Some(throw) = visit_recommendation[1] {
                    if throw.multiplier == 2 {
                        "border-b-brand-secondary"
                    } else if throw.multiplier == 3 {
                        "border-b-brand-primary"
                    } else {
                        "border-b-brand-text"
                    }
                } else {
                    "border-b-brand-text"
                }
            )}>
            <span class="text-sm font-bold text-brand-bg">
                {
                    if let Some(throw) = visit_recommendation[1] {
                        format!("{}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            <td class={format!("w-1/3 h-8 border-l border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center {}",
                if let Some(throw) = visit_recommendation[2] {
                    if throw.multiplier == 2 {
                        "border-b-brand-secondary"
                    } else if throw.multiplier == 3 {
                        "border-b-brand-primary"
                    } else {
                        "border-b-brand-text"
                    }
                } else {
                    "border-b-brand-text"
                }
            )}>
            <span class="text-sm font-bold text-brand-bg">
                {
                    if let Some(throw) = visit_recommendation[2] {
                        format!("{}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            </tr>
        </tbody>
        </table>
        <div class="h-1"></div>
            <table class="text-brand-text text-sm w-full">
                <tr>
                    <td>{"3 Dart-Avg. (Leg)"}</td>
                    <td>{format!("{:.2}", leg_average)}</td>
                </tr>
                <tr>
                    <td>{"3 Dart-Avg. (Set)"}</td>
                    <td>{format!("{:.2}", set_average)}</td>
                </tr>
                <tr>
                    <td>{"3 Dart-Avg."}</td>
                    <td>{format!("{:.2}", average)}</td>
                </tr>
                <tr>
                    <td>{"Last Throw"}</td>
                    <td>{last_visit_score}</td>
                </tr>
                <tr>
                    <td>{"Thrown Darts"}</td>
                    <td>{darts_thrown}</td>
                </tr>
            </table>
        </div>
    }
}
