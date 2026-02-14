use yew::prelude::*;
use crate::components::{ScoreInput, ScoreBoard};
use crate::models::{GameState, Route, RouteState, RouteAction};
use crate::log_error;

#[function_component(GameView)]
pub fn game_view() -> Html {
    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("GameState context not found - cannot render GameView");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let Some(route_state) = use_context::<UseReducerHandle<RouteState>>() else {
        log_error!("RouteState context not found - cannot render GameView");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    use_effect_with(game_state.winner, move |winner| {
        log_error!("Checking winner: {:?}", winner);
        if winner.is_some() {
            route_state.dispatch(RouteAction::Navigate(Route::Win));
        }
    });

    html! {
        <div>
            <ScoreBoard />
            <div class="flex-1"></div>
            <ScoreInput />
        </div>
    }
}
