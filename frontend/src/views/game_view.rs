use yew::prelude::*;
use crate::components::{ScoreInput, ScoreBoard};

#[function_component(GameView)]
pub fn game_view() -> Html {
    html! {
        <>
            <ScoreBoard />
            <div class="flex-1"></div>
            <ScoreInput />
        </>
    }
}
