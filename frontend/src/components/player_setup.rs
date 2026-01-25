use yew::prelude::*;

#[function_component(PlayerSetup)]
pub fn player_setup() -> Html {
    html! {
        <div class="w-full mb-8">
            <h2 class="text-2xl text-brand-text font-bold mb-4 text-center">{ "Players" }</h2>

            <div class="flex items-center justify-center mb-6">
                <div class="relative flex items-center w-full max-w-[200px]">
                    <button class="absolute left-0 w-10 h-10 bg-brand-bg border-2 border-brand-text text-brand-text rounded-l">
                        <i class="ti ti-minus"></i>
                    </button>
                    <input
                        type="number"
                        readonly=true
                        value="2"
                        class="w-full h-10 text-center bg-brand-bg border-y-2 border-brand-text text-brand-text font-bold text-xl focus:outline-none"
                    />
                    <button class="absolute right-0 w-10 h-10 bg-brand-bg border-2 border-brand-text text-brand-text rounded-r">
                        <i class="ti ti-plus"></i>
                    </button>
                </div>
            </div>

            <div class="space-y-3">
                <div class="relative">
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <i class="ti ti-user text-brand-text"></i>
                    </div>
                    <input
                        type="text"
                        placeholder="Player 1"
                        class="w-full bg-brand-bg border-2 border-brand-primary text-brand-text rounded py-2 pl-10 pr-4 focus:outline-none focus:border-brand-secondary placeholder-brand-text"
                    />
                </div>
            </div>
        </div>
    }
}
