use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component(JoinForm)]
pub fn join_form() -> Html {
    let code_input = use_state(|| String::new());

    let on_code_change = {
        let code_input = code_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut value = input.value().to_uppercase();
            if value.len() > 5 {
                value.truncate(5);
                input.set_value(&value);
            }
            code_input.set(value);
        })
    };

    let on_enter = {
        let code_input = code_input.clone();
        Callback::from(move |_: MouseEvent| {
            if code_input.len() == 5 {
                // TODO: Handle join with code
            }
        })
    };

    html! {
        <div class="w-full">
        <div class="text-left p-3 text-2xl flex justify-between items-center border-b border-brand-text">
            <table class="w-full">
                <tr>
                    <td class="flex-1">
                        { "Join Game" }
                    </td>
                </tr>
            </table>
        </div>
        <div class="flex flex-col gap-4">
            <div class="relative text-brand-text border-brand-text focus-within:text-brand-primary">
            <table class="w-full">
            <tr>
            <td class="w-10 px-3 py-2 pointer-events-none align-middle">
                <i class="ti ti-hash"></i>
            </td>
                <td class="align-middle">
                    <div class="flex items-center">
                        <input
                            type="text"
                            placeholder={"Game Code"}
                            maxlength="5"
                            class="w-full py-2 bg-brand-bg rounded-r-none text-brand-text focus:text-brand-primary focus:outline-none placeholder-brand-text/50 focus:placeholder-brand-primary/50 uppercase"
                            value={(*code_input).clone()}
                            oninput={on_code_change}
                        />
                        <button
                            onclick={on_enter}
                            disabled={(*code_input).len() != 5}
                            class="px-4 py-2 bg-brand-bg text-brand-text rounded-r-xl border-l border-brand-text hover:text-brand-primary disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <i class="ti ti-arrow-right"></i>
                        </button>
                    </div>
                </td>
            </tr>
            </table>
            </div>
            </div>
        </div>
    }
}
