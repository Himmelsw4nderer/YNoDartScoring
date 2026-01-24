use yew::prelude::*;
use crate::models::{Throw, ThrowState, ThrowAction, LegAction, LegState};
use crate::utils::translate_multiplier_to_char;
use crate::{log_error};

#[function_component(ScoreInput)]
pub fn score_input() -> Html {
    let Some(throw_state) = use_context::<UseReducerHandle<ThrowState>>() else {
        log_error!("ThrowState context not found - cannot render ScoreInput");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let Some(leg_state) = use_context::<UseReducerHandle<LegState>>() else {
        log_error!("LegState context not found - cannot render ScoreInput");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let active_multiplier = use_state(|| 1);
    let edit_throw_index = use_state(|| None::<usize>);
    let current_throw_index = use_state(|| 0);


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
        let throw_state = throw_state.clone();
        let current_throw_index = current_throw_index.clone();
        let edit_throw_index = edit_throw_index.clone();

        move |n: i32| {
            let index = edit_throw_index.as_ref().copied().unwrap_or(*current_throw_index);

            if index >= 3 {
                active_multiplier.set(1);
                return;
            }

            let throw = Throw {
                field: n,
                multiplier: *active_multiplier,
            };

            throw_state.dispatch(ThrowAction::AddThrow(index, throw));

            if edit_throw_index.is_none() {
                current_throw_index.set(index + 1);
            } else {
                edit_throw_index.set(None);
            }

            active_multiplier.set(1);
        }
    };

    let on_throw_click = {
        let throw_state = throw_state.clone();
        let edit_throw_index = edit_throw_index.clone();
        move |index: usize| {
            let throws = throw_state.current_throws;
            let edit_throw_index = edit_throw_index.clone();

            Callback::from(move |_| {
                if throws[index].is_none() {
                    return;
                }
                edit_throw_index.set(Some(index));
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
        let throw_state = throw_state.clone();
        let leg_state = leg_state.clone();
        let current_throw_index = current_throw_index.clone();
        let edit_throw_index = edit_throw_index.clone();

        Callback::from(move |_| {
            leg_state.dispatch(LegAction::ChangeThrow(
                leg_state.current_player,
                None,
                (*throw_state).clone()
            ));
            leg_state.dispatch(LegAction::NextPlayer);
            throw_state.dispatch(ThrowAction::Clear);
            current_throw_index.set(0);
            edit_throw_index.set(None);
        })
    };

    html! {
        <div class="mt-8 w-full">
        <table class="w-full bg-brand-text p-1">
        <tbody>
            <tr>
            <td class={format!("w-1/4 h-16 border-r border-brand-bg border-t-brand-text border-t-4 border-b-4 text-center cursor-pointer {}",
                if let Some(throw) = throw_state.current_throws[0].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[0].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[0].as_ref() {
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
                if let Some(throw) = throw_state.current_throws[1].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[1].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[1].as_ref() {
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
                if let Some(throw) = throw_state.current_throws[2].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[2].as_ref() {
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
                    if let Some(throw) = throw_state.current_throws[2].as_ref() {
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
            <button class="p-3 p-y-5 w-full h-full text-center text-brand-bg text-xl" onclick={on_submit_throw}>{ "Submit" }</button>
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
                                "p-3 p-y-5 w-1/5 cursor-pointer border-b-4 {}",
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
                                "p-3 p-y-5 w-1/5 cursor-pointer border-b-4 {}",
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
                                "p-3 p-y-5 w-1/5 border-b-4 border-b-transparent {}",
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
                                "p-3 p-y-5 w-1/5 border-b-4 border-b-transparent {}",
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
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(1)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "1" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(2)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "2" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(3)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "3" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(4)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "4" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
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
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(6)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "6" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(7)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "7" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(8)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "8" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(9)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "9" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
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
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(11)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "11" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(12)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "12" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(13)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "13" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(14)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "14" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
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
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(16)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "16" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(17)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "17" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(18)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "18" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
                            onclick={
                                let cb = on_score_click.clone();
                                Callback::from(move |_| cb.emit(Button::Number(19)))
                            }
                        >
                            <button class="w-full h-full text-center text-brand-text">{ "19" }</button>
                        </td>
                        <td
                            class="py-5 px-3 w-1/5 cursor-pointer"
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
