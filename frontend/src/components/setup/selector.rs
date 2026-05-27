use yew::prelude::*;
use crate::log_error;
use crate::models::{SetupState, SetupAction};

#[derive(Properties, PartialEq)]
pub struct SelectorProps {
    pub title: String,
    pub values: Vec<i32>,
    pub selected_value: i32,
    pub on_select: Callback<i32>,
}

#[function_component(Selector)]
pub fn selector(props: &SelectorProps) -> Html {
    html! {
        <div class="w-full mb-8">
            <h2 class="text-2xl text-brand-text font-bold mb-4 text-center">{ &props.title }</h2>
            <div class="grid grid-cols-4 gap-2">
                {
                    props.values.iter().map(|value| {
                        let is_selected = *value == props.selected_value;
                        let bg_class = if is_selected { "bg-brand-primary" } else { "bg-brand-bg" };
                        let border_class = if is_selected { "border-brand-primary" } else { "border-brand-text" };

                        let onclick = {
                            let on_select = props.on_select.clone();
                            let value = *value;
                            Callback::from(move |_| {
                                on_select.emit(value);
                            })
                        };

                        html! {
                            <button {onclick} class={format!("py-3 rounded text-xl font-bold border-2 {} {} text-brand-text transition-colors", bg_class, border_class)}>
                                { *value }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[function_component(FirstToSetsSelector)]
pub fn first_to_sets_selector() -> Html {
    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render FirstToSelector");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let first_to_sets_values = vec![1, 3, 5, 7];
    let selected_value = setup_state.set_goal;

    let on_select = {
        let setup_state = setup_state.clone();
        Callback::from(move |value| {
            setup_state.dispatch(SetupAction::SetSetGoal(value));
        })
    };

    html! {
        <Selector
            title={"First to Set"}
            values={first_to_sets_values}
            selected_value={selected_value}
            on_select={on_select}
        />
    }
}

#[function_component(FirstToLegsSelector)]
pub fn first_to_legs_selector() -> Html {
    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render FirstToLegsSelector");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let first_to_legs_values = vec![1, 2, 3, 5];
    let selected_value = setup_state.leg_goal;

    let on_select = {
        let setup_state = setup_state.clone();
        Callback::from(move |value| {
            setup_state.dispatch(SetupAction::SetLegGoal(value));
        })
    };

    html! {
        <Selector
            title={"First to Legs"}
            values={first_to_legs_values}
            selected_value={selected_value}
            on_select={on_select}
        />
    }
}

#[function_component(StartingScoreSelector)]
pub fn starting_score_selector() -> Html {
    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render StartingScoreSelector");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let starting_score_values = vec![301, 501, 701, 901];
    let selected_value = setup_state.starting_score;

    let on_select = {
        let setup_state = setup_state.clone();
        Callback::from(move |value| {
            setup_state.dispatch(SetupAction::SetStartingScore(value));
        })
    };

    html! {
        <Selector
            title={"Starting Score"}
            values={starting_score_values}
            selected_value={selected_value}
            on_select={on_select}
        />
    }
}
