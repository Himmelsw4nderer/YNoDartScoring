use yew::prelude::*;
use crate::components::{ScoreInput, ScoreBoard};
use crate::models::{LegState, Route, RouteState, RouteAction};
use crate::log_error;

#[function_component(GameView)]
pub fn game_view() -> Html {
    let Some(leg_state) = use_context::<UseReducerHandle<LegState>>() else {
        log_error!("LegState context not found - cannot render GameView");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let Some(route_state) = use_context::<UseReducerHandle<RouteState>>() else {
        log_error!("RouteState context not found - cannot render GameView");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    use_effect_with(leg_state.winner, move |winner| {
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
