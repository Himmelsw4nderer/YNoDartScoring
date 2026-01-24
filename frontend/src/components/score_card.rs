use yew::prelude::*;
use crate::utils::translate_multiplier_to_char;
use crate::checkout::recommend_throws;
use crate::models::{LegState, ThrowState};
use crate::log_error;


#[derive(Properties, PartialEq)]
pub struct ScoreCardProps {
    pub player_index: usize,
}

#[function_component(ScoreCard)]
pub fn score_card(props: &ScoreCardProps) -> Html {
    let Some(throw_state) = use_context::<UseReducerHandle<ThrowState>>() else {
        log_error!("ThrowState context not found - cannot render ScoreCard");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let Some(leg_state) = use_context::<UseReducerHandle<LegState>>() else {
        log_error!("LegState context not found - cannot render ScoreCard");
        return html! { <div>{"Error: Context not available"}</div> };
    };
    let player = &leg_state.player_states[props.player_index];
    let is_turn = leg_state.current_player == props.player_index;
    let mut is_bust = false;

    let mut score = player.score;

    if let Some(throw_score) = throw_state.get_clean_score(score) {
        score -= throw_score;
    } else if is_turn{
        is_bust = true;
    }

    if !is_turn{
        score = player.score;
    }

    let current_throw_if_turn = if is_turn { Some((*throw_state).clone()) } else { None };
    let recommended_throws = if is_bust { throw_state.current_throws.clone() } else { recommend_throws(score, current_throw_if_turn.clone()) };
    let average = player.calculate_average(&current_throw_if_turn);


    let started = leg_state.starting_player == props.player_index;

    let last_throw = player.throw_states.last().map(|ts| ts.get_score()).unwrap_or(0);
    let darts_thrown = player.throw_states.len() as i32 * 3 + throw_state.get_throw_amount();

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
                if let Some(throw) = recommended_throws[0] {
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
                    if let Some(throw) = recommended_throws[0] {
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
                if let Some(throw) = recommended_throws[1] {
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
                    if let Some(throw) = recommended_throws[1] {
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
                if let Some(throw) = recommended_throws[2] {
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
                    if let Some(throw) = recommended_throws[2] {
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
                    <td>{"3 Dart-Avg."}</td>
                    <td>{format!("{:.2}", average)}</td>
                </tr>
                <tr>
                    <td>{"Last Throw"}</td>
                    <td>{last_throw}</td>
                </tr>
                <tr>
                    <td>{"Thrown Darts"}</td>
                    <td>{darts_thrown}</td>
                </tr>
            </table>
        </div>
    }
}
