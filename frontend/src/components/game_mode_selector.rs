use yew::prelude::*;

#[function_component(GameModeSelector)]
pub fn game_mode_selector() -> Html {
    let modes = vec!["301", "501", "701", "901"];
    let selected_mode = "501";

    html! {
        <div class="w-full mb-8">
            <h2 class="text-2xl text-brand-text font-bold mb-4 text-center">{ "Starting Score" }</h2>
            <div class="grid grid-cols-4 gap-2">
                {
                    modes.into_iter().map(|mode| {
                        let is_selected = mode == selected_mode;
                        let bg_class = if is_selected { "bg-brand-primary" } else { "bg-brand-bg" };
                        let border_class = if is_selected { "border-brand-primary" } else { "border-brand-text" };

                        html! {
                            <button class={format!("py-3 rounded text-xl font-bold border-2 {} {} text-brand-text", bg_class, border_class)}>
                                { mode }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
