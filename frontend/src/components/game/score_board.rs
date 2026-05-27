use yew::prelude::*;
use crate::components::game::score_card::ScoreCard;
use crate::components::game::collapsed_score_card::CollapsedScoreCard;
use crate::models::GameState;
use crate::log_error;

#[function_component(ScoreBoard)]
pub fn score_board() -> Html {
    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("GameState context not found - cannot render ScoreBoard");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let player_count = game_state.players.len();

    if player_count == 0 {
        html! {
            <div class="score-board flex gap-4">
                <div>{"No players configured"}</div>
            </div>
        }
    } else if player_count == 1 {
        html! {
            <div class="score-board flex gap-4">
                <div class="w-full"><ScoreCard player_index={0} /></div>
            </div>
        }
    } else if player_count == 2 {
        html! {
            <div class="score-board flex gap-4">
                <div class="w-1/2"><ScoreCard player_index={0} /></div>
                <div class="w-1/2"><ScoreCard player_index={1} /></div>
            </div>
        }
    } else {
        html! {
            <div class="score-board flex gap-4">
                <div class="w-1/2"><ScoreCard player_index={game_state.current_player} /></div>
                <div class="w-full flex flex-wrap gap-2">
                    { for game_state.players.iter().enumerate().filter(|(i, _)| *i != game_state.current_player).map(|(i, _)| html! {
                        <CollapsedScoreCard player_index={i} />
                    }) }
                </div>
            </div>
        }
    }
}
