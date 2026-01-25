use yew::prelude::*;
use crate::hooks::use_navigator;
use crate::models::Route;

#[function_component(WinView)]
pub fn win_view() -> Html {
    let navigator = use_navigator();

    html! {
        <div class="win-view">
            <h1>{""}</h1>
            <button onclick={navigator(Route::Home)}>{"Back"}</button>
        </div>
    }
}
