use yew::prelude::*;
use crate::models::{GameState, Throw, GameAction};
use crate::utils::translate_multiplier_to_char;
use crate::{log_error};

#[function_component(ScoreInput)]
pub fn score_input() -> Html {
    let Some(game_state) = use_context::<UseReducerHandle<GameState>>() else {
        log_error!("LegState context not found - cannot render ScoreInput");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let active_multiplier = use_state(|| 1);
    let edit_throw_index = use_state(|| None::<usize>);

    #[derive(Clone, PartialEq)]
    enum Button {
        Multiplier(i32),
        Number(i32),
    }

    let handle_multiplier = {
        let active_multiplier = active_multiplier.clone();
        move |m: i32| {
            active_multiplier.set(m);
        }
    };

    let handle_number = {
        let active_multiplier = active_multiplier.clone();
        let edit_throw_index = edit_throw_index.clone();
        let game_state = game_state.clone();


        move |n: i32| {
            let game_state = game_state.clone();

            let throw = Throw {
                field: n,
                multiplier: *active_multiplier,
            };

            game_state.dispatch(GameAction::ChangeThrow(throw, None, None, None, None, edit_throw_index.as_ref().map(|i| *i)));

            if ! edit_throw_index.is_none() {
                edit_throw_index.set(None);
            }

            active_multiplier.set(1);
        }
    };

    let on_throw_click = {
        let edit_throw_index = edit_throw_index.clone();
        let game_state = game_state.clone();

        move |throw_index: usize| {
            let edit_throw_index = edit_throw_index.clone();
            let game_state = game_state.clone();

            Callback::from(move |_| {
                if game_state.get_throw(None, None, None, None, Some(throw_index)).is_none() {
                    return;
                }
                edit_throw_index.set(Some(throw_index));
            })
        }
    };

    let on_score_click = {
        let handle_multiplier = handle_multiplier.clone();
        let handle_number = handle_number.clone();
        Callback::from(move |score: Button| {
            match score {
                Button::Multiplier(m) => handle_multiplier(m),
                Button::Number(n) => handle_number(n),
            }
        })
    };

    let on_submit_throw = {
        let edit_throw_index = edit_throw_index.clone();
        let game_state = game_state.clone();

        Callback::from(move |_| {
            game_state.dispatch(GameAction::NextPlayer);
            edit_throw_index.set(None);
        })
    };

    html! {
        <div class="mt-8 w-full">
        <table class="w-full bg-brand-text p-1">
        <tbody>
            <tr>
            <td class={format!("w-1/4 h-16 border-r border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center cursor-pointer {}",
                if let Some(throw) = game_state.get_throw(None, None, None, None, Some(0)).as_ref() {
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
            )}
            onclick={on_throw_click(0)}>
            <span class={format!("text-3xl font-bold text-brand-bg {}", if *edit_throw_index == Some(0) { "underline" } else { "" })}>
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(0)).as_ref() {
                        format!("{}",
                            throw.get_score()
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            <span class="text-base opacity-60 text-brand-bg">
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(0)).as_ref() {
                        format!(" {}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            <td class={format!("w-1/4 h-16 border-x border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center cursor-pointer {}",
                if let Some(throw) = game_state.get_throw(None, None, None, None, Some(1)).as_ref() {
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
            )}
            onclick={on_throw_click(1)}>
            <span class={format!("text-3xl font-bold text-brand-bg {}", if *edit_throw_index == Some(1) { "underline" } else { "" })}>
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(1)).as_ref() {
                        format!("{}",
                            throw.get_score()
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            <span class="text-base opacity-60 text-brand-bg">
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(1)).as_ref() {
                        format!(" {}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            <td class={format!("w-1/4 h-16 border-x border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center cursor-pointer {}",
                if let Some(throw) = game_state.get_throw(None, None, None, None, Some(2)).as_ref() {
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
            )}
            onclick={on_throw_click(2)}>
            <span class={format!("text-3xl font-bold text-brand-bg {}", if *edit_throw_index == Some(2) { "underline" } else { "" })}>
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(2)).as_ref() {
                        format!("{}",
                            throw.get_score()
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            <span class="text-base opacity-60 text-brand-bg">
                {
                    if let Some(throw) = game_state.get_throw(None, None, None, None, Some(2)).as_ref() {
                        format!(" {}{}",
                            translate_multiplier_to_char(throw.multiplier),
                            throw.field
                        )
                    } else {
                        String::new()
                    }
                }
            </span>
            </td>
            <td class="w-1/4 text-right text-brand-bg order-l border-brand-bg">
            <button class="p-3 w-full h-full text-center text-brand-bg text-xl" onclick={on_submit_throw}>{ "Submit" }</button>
            </td>
            </tr>
        </tbody>
        </table>

            <div class="h-2"></div>

            <table class="w-full border-collapse table-fixed text-xl">
                <tbody>
                    <tr>
                        <td
                            class={format!(
                                "p-3 w-1/5 cursor-pointer border-b-4 {}",
                                if *active_multiplier == 1 { "border-b-brand-text" } else { "border-b-transparent" }
                            )}
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Multiplier(1)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "Single" }</button>
                        </td>
                        <td
                            class={format!(
                                "p-3 w-1/5 cursor-pointer border-b-4 {}",
                                if *active_multiplier == 2 { "border-b-brand-secondary" } else { "border-b-transparent" }
                            )}
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Multiplier(2)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "Double" }</button>
                        </td>
                        <td
                            class={format!(
                                "p-1 w-1/5 cursor-pointer border-b-4 {}",
                                if *active_multiplier == 3 { "border-b-brand-primary" } else { "border-b-transparent" }
                            )}
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Multiplier(3)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "Triple" }</button>
                        </td>
                        <td
                            class={format!(
                                "p-3 w-1/5 border-b-4 border-b-transparent {}",
                                if *active_multiplier == 3 { "cursor-not-allowed opacity-50" } else { "cursor-pointer" }
                            )}
                            onclick={
                                let cb = on_score_click.clone();
                                let active_multiplier = active_multiplier.clone();
                                Callback::from(move |_| {
                                    if *active_multiplier != 3 {
                                        cb.emit(Button::Number(25))
                                    }
                                })
                            }
                        >
                            <button
                                class="w-full h-full text-center text-brand-text"
                                disabled={*active_multiplier == 3}
                            >
                                { "Bull" }
                            </button>
                        </td>
                        <td
                            class={format!(
                                "p-3 w-1/5 border-b-4 border-b-transparent {}",
                                if *active_multiplier == 1 { "cursor-pointer" } else { "cursor-not-allowed opacity-50" }
                            )}
                            onclick={
                                let cb = on_score_click.clone();
                                let active_multiplier = active_multiplier.clone();
                                Callback::from(move |_| {
                                    if *active_multiplier == 1 {
                                        cb.emit(Button::Number(0))
                                    }
                                })
                            }
                        >
                            <button
                                class="w-full h-full text-center text-brand-text"
                                disabled={*active_multiplier != 1}
                            >
                                { "Out" }
                            </button>
                        </td>
                    </tr>

                    <tr>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(1)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "1" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(2)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "2" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(3)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "3" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(4)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "4" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(5)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "5" }</button>
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(6)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "6" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(7)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "7" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(8)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "8" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(9)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "9" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(10)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "10" }</button>
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(11)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "11" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(12)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "12" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(13)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "13" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(14)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "14" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(15)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "15" }</button>
                        </td>
                    </tr>
                    <tr>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(16)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "16" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(17)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "17" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(18)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "18" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(19)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "19" }</button>
                        </td>
                        <td
                            class="p-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(20)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "20" }</button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
