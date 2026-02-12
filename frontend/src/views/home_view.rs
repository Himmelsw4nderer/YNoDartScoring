use yew::prelude::*;
use crate::models::Route;
use crate::hooks::use_navigator;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    let navigator = use_navigator();


    html! {
        <table class="border-separate border-spacing-1">
            <tr>
                <td class="w-1/2 p-0">
                    <button onclick={navigator(Route::Setup)} class="w-full h-full bg-brand-primary text-brand-text rounded-tl-2xl flex flex-col">
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
                    <button disabled={true} class="h-full w-full bg-brand-secondary text-brand-text rounded-tr-2xl flex flex-col">
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
            <tr>
                <td class="p-0" colspan="3">
                    <button disabled={true} class="w-full bg-brand-bg text-brand-text border border-brand-text">
                        <div class="text-left p-3 text-2xl flex justify-between items-center">
                            <table class="w-full">
                                <tr>
                                    <td class="flex-1">
                                        {"Statistics"}
                                        <div class="text-left text-sm">
                                            {"Check your progress."}
                                        </div>
                                    </td>
                                    <td class="text-right"><i class="ti ti-graph text-5xl"></i></td>
                                </tr>
                            </table>
                        </div>
                    </button>
                </td>
            </tr>
            <tr>
                <td class="p-0" colspan="3">
                    <button disabled={true} class="w-full bg-brand-bg text-brand-text border border-brand-text rounded-b-lg">
                        <div class="text-left p-3 text-2xl flex justify-between items-center">
                            <table class="w-full">
                                <tr>
                                    <td class="text-left flex-1"><i class="ti ti-settings text-5xl"></i></td>
                                    <td class="text-right">
                                        {"Settings"}
                                        <div class="text-right text-sm">
                                            {"Customize your experience."}
                                        </div>
                                    </td>
                                </tr>
                            </table>
                        </div>
                    </button>
                </td>
            </tr>
        </table>
    }
}
