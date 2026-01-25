use yew::prelude::*;
use crate::models::{Route, RouteAction, RouteState};

#[hook]
pub fn use_navigator() -> impl Fn(Route) -> Callback<MouseEvent> {
    let route_state = use_context::<UseReducerHandle<RouteState>>()
        .expect("RouteState context to be present");

    move |route: Route| {
        let route_state = route_state.clone();
        Callback::from(move |_: MouseEvent| {
            route_state.dispatch(RouteAction::Navigate(route.clone()));
        })
    }
}
