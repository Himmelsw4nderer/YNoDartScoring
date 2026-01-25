use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::{LegState, LegAction};
use crate::log_error;

#[function_component(PlayerSetup)]
pub fn player_setup() -> Html {
    let Some(leg_state) = use_context::<UseReducerHandle<LegState>>() else {
        log_error!("LegState context not found - cannot render PlayerSetup");
        return html! { <div>{"Error: Context not available"}</div> };
    };

    let player_count = leg_state.player_states.len();

    let on_add_player = {
        let leg_state = leg_state.clone();
        Callback::from(move |_| {
            leg_state.dispatch(LegAction::AddPlayer);
        })
    };

    let on_remove_player = {
        let leg_state = leg_state.clone();
        let player_count = player_count;
        Callback::from(move |_| {
            if player_count > 1 {
                leg_state.dispatch(LegAction::RemovePlayer(player_count - 1));
            }
        })
    };

    html! {
        <div class="w-full mb-8">
            <h2 class="text-2xl text-brand-text font-bold mb-4 text-center">{ "Players" }</h2>

            <div class="flex items-center justify-center mb-6">
                <div class="relative flex items-center w-full max-w-[200px]">
                    <button
                        onclick={on_remove_player}
                        disabled={player_count <= 2|| true}
                        class="absolute left-0 w-10 h-10 bg-brand-bg border-2 border-brand-text text-brand-text"
                    >
                        <i class="ti ti-minus"></i>
                    </button>
                    <input
                        type="number"
                        readonly=true
                        value={player_count.to_string()}
                        class="w-full h-10 text-center bg-brand-bg border-y-2 border-brand-text text-brand-text font-bold text-xl focus:outline-none"
                    />
                    <button
                        onclick={on_add_player}
                        disabled={player_count >= 10 || true}
                        class="absolute right-0 w-10 h-10 bg-brand-bg border-2 border-brand-text text-brand-text"
                    >
                        <i class="ti ti-plus"></i>
                    </button>
                </div>
            </div>

            <div class="space-y-3">
                {
                    leg_state.player_states.iter().enumerate().map(|(index, player)| {
                        let on_name_change = {
                            let leg_state = leg_state.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                leg_state.dispatch(LegAction::SetPlayerName(index, input.value()));
                            })
                        };

                        html! {
                            <div class="relative" key={index}>
                                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                    <i class="ti ti-user text-brand-text"></i>
                                </div>
                                <input
                                    type="text"
                                    value={player.name.clone()}
                                    oninput={on_name_change}
                                    placeholder={format!("Player {}", index + 1)}
                                    class="w-full bg-brand-bg border-2 border-brand-text text-brand-text rounded py-2 pl-10 pr-4 focus:outline-none focus:border-brand-primary placeholder-brand-text/50"
                                />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
