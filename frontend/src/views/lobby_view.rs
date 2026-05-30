use yew::prelude::*;
use crate::hooks::use_navigator;

#[function_component(LobbyView)]
pub fn lobby_view() -> Html {
    let navigator = use_navigator();

    html! {
        <div>
            <table class="border-separate border-spacing-1">
                <tr>
                    <td class="w-1/2 p-0">
                        <button onclick={navigator(crate::models::Route::Setup)} class="w-full h-full bg-brand-primary text-brand-text rounded-tl-2xl flex flex-col">
                            <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                                <table class="w-full">
                                    <tr>
                                        <td class="flex-1">
                                            {"Play"}
                                            <br/>
                                            {"Local"}
                                        </td>
                                        <td class="text-right"><i class="ti ti-users text-6xl"></i></td>
                                    </tr>
                                </table>
                            </div>
                            <div class="text-left p-3 text-sm h-16 flex items-center">
                                {"Game with a friend on the same device."}
                            </div>
                        </button>
                    </td>
                    <td class="w-1/2 p-0">
                        <button onclick={navigator(crate::models::Route::OnlineLobby)} class="h-full w-full bg-brand-secondary text-brand-text rounded-tr-2xl flex flex-col hover:opacity-90 transition-opacity">
                            <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                                <table class="w-full">
                                    <tr>
                                        <td class="flex-1">
                                            {"Play"}
                                            <br/>
                                            {"Online"}
                                        </td>
                                        <td class="text-right"><i class="ti ti-world text-6xl"></i></td>
                                    </tr>
                                </table>
                            </div>
                            <div class="text-left p-3 text-sm h-16 flex items-center">
                                {"Game with another players across the world."}
                            </div>
                        </button>
                    </td>
                </tr>
            </table>

            <div class="flex justify-center mt-4">
                <button
                    onclick={navigator(crate::models::Route::Home)}
                    class="px-6 py-2 bg-brand-bg text-brand-text border border-brand-text rounded-lg hover:opacity-90 transition-opacity"
                >
                    <i class="ti ti-arrow-left mr-2"></i>
                    {"Back to Menu"}
                </button>
            </div>
        </div>
    }
}
