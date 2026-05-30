use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::{SetupState, SetupAction};
use crate::log_error;

#[function_component(PlayerSetup)]
pub fn player_setup() -> Html {
    let Some(setup_state) = use_context::<UseReducerHandle<SetupState>>() else {
        log_error!("SetupState context not found - cannot render PlayerSetup");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let player_count = setup_state.players.len();

    let on_add_player = {
        let setup_state = setup_state.clone();
        Callback::from(move |_| {
            setup_state.dispatch(SetupAction::AddPlayer());
        })
    };

    let on_remove_player = {
        let setup_state = setup_state.clone();
        Callback::from(move |_| {
            setup_state.dispatch(SetupAction::RemovePlayer());
        })
    };

    html! {
        <div class="w-full pt-1">
            <div class="text-left  text-2xl flex justify-between items-center">
                <table class="w-full text-brand-text">
                    <tr>
                    <td class="flex-1 p-3">
                        { "Players" }
                    </td>
                    <td class="flex-1 text-right">
                        <div class="flex items-center justify-end h-full">
                            <div class="relative flex items-center w-full h-full">
                                <button
                                    onclick={on_remove_player}
                                    disabled={player_count <= 1}
                                    class="absolute left-0 w-10 h-10 bg-brand-bg border-x border-brand-text text-brand-text"
                                >
                                    <i class="ti ti-minus"></i>
                                </button>
                                <input
                                    type="number"
                                    readonly=true
                                    value={player_count.to_string()}
                                    class="w-full h-10 text-center bg-brand-bg border-brand-text text-brand-text font-bold text-xl focus:outline-none"
                                />
                                <button
                                    onclick={on_add_player}
                                    disabled={player_count >= 10}
                                    class="absolute right-0 w-10 h-10 bg-brand-bg border-l border-brand-text text-brand-text"
                                >
                                    <i class="ti ti-plus"></i>
                                </button>
                            </div>
                        </div>
                        </td>
                    </tr>
                </table>
            </div>

            <div class="">
                {
                    setup_state.players.iter().enumerate().map(|(index, player)| {
                        let on_name_change = {
                            let setup_state = setup_state.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                setup_state.dispatch(SetupAction::RenamePlayer(index, input.value()));
                            })
                        };

                        html! {
                            <div class="relative bg-brand-bg border-t text-brand-text border-brand-text focus-within:border-brand-primary focus-within:text-brand-primary" key={index}>
                            <table class="w-full">
                                <tr>
                                <td class="w-10 px-3 py-2 pointer-events-none align-middle">
                                    <i class="ti ti-user"></i>
                                </td>
                                    <td class="align-middle">
                                        <input
                                            type="text"
                                            value={player.name.clone()}
                                            oninput={on_name_change}
                                            placeholder={format!("Player {}", index + 1)}
                                            class="w-full py-2 bg-brand-bg text-brand-text focus:outline-none placeholder-brand-text/50"
                                        />
                                    </td>
                                </tr>
                            </table>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
