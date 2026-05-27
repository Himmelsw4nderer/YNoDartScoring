use yew::prelude::*;
use crate::components::game::score_card::ScoreCard;

#[function_component(ScoreBoard)]
pub fn score_board() -> Html {
    html! {
        <div class="score-board flex gap-4">
            <ScoreCard player_index={0} />
            <ScoreCard player_index={1} />
        </div>
    }
}
