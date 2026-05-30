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
        <div class="w-full">
        <div class="text-left p-3 text-2xl flex justify-between items-center border-b border-brand-text">
            <table class="w-full">
                <tr>
                    <td class="flex-1">
                        { "Players" }
                    </td>
                </tr>
            </table>
        </div>
        <div class="flex">
            <div class="flex flex-col gap-0">
                <button
                    onclick={on_add_player}
                    disabled={player_count >= 4}
                    class="w-10 h-10 bg-brand-bg border-r border-brand-text text-brand-text flex items-center justify-center hover:bg-brand-primary/10 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    <i class="ti ti-plus"></i>
                </button>
                <div class="flex-1 flex items-center justify-center bg-brand-bg border-t border-r border-brand-text text-brand-text font-bold text-lg w-10">
                    {player_count}
                </div>
                <button
                    onclick={on_remove_player}
                    disabled={player_count <= 1}
                    class="w-10 h-10 bg-brand-bg border-t border-r border-brand-text text-brand-text rounded-bl-xl flex items-center justify-center hover:bg-brand-primary/10 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    <i class="ti ti-minus"></i>
                </button>
            </div>
            <div class="flex-1">
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
                            <div class="relative text-brand-text border-brand-text focus-within:text-brand-primary" key={index}>
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
                                            class="w-full py-2 bg-brand-bg rounded-r-xl text-brand-text focus:text-brand-primary focus:outline-none placeholder-brand-text/50 focus:placeholder-brand-primary/50"
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
        </div>
    }
}
