use yew::prelude::*;
use crate::hooks::use_navigator;
use crate::models::Route;
use crate::components::JoinForm;

#[function_component(SetupJoinView)]
pub fn setup_join_view() -> Html {
    let navigator = use_navigator();

    html! {
        <div>
            <table class="border-separate border-spacing-1 w-full">
                <tr>
                <td colspan=1 class="w-1/3 p-0">
                    <button onclick={navigator(Route::Home)} class="w-full h-full bg-brand-primary text-brand-text rounded-tl-2xl flex flex-col">
                        <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                            <table class="w-full">
                                <tr>
                                    <td class="flex-1">
                                        {"Go"}
                                        <br/>
                                        {"Back"}
                                    </td>
                                    <td class="text-right"><i class="ti ti-arrow-back text-6xl"></i></td>
                                </tr>
                            </table>
                        </div>
                        <div class="text-left p-3 text-sm h-16 flex items-center">
                            {""}
                        </div>
                    </button>
                </td>
                    <td class="w-2/3 p-0">
                        <button onclick={navigator(Route::SetupLobby)} class="h-full w-full bg-brand-secondary text-brand-text rounded-tr-2xl flex flex-col hover:opacity-90 transition-opacity">
                            <div class="text-left p-3 font-bold text-2xl flex justify-between items-center">
                                <table class="w-full">
                                    <tr>
                                        <td class="flex-1">
                                            {"Host a"}
                                            <br/>
                                            {"Game"}
                                        </td>
                                        <td class="text-right"><i class="ti ti-world text-6xl"></i></td>
                                    </tr>
                                </table>
                            </div>
                            <div class="text-left p-3 text-sm h-16 flex items-center">
                                {"Host a game for players to join from anywhere."}
                            </div>
                        </button>
                    </td>
                </tr>
                <tr class="p-0">
                    <td colspan=2 class="p-0 m-0"><div class="w-full bg-brand-bg text-brand-text border border-brand-text rounded-b-lg">
                        <JoinForm />
                    </div></td>
                </tr>
            </table>
        </div>
    }
}
