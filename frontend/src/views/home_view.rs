use yew::prelude::*;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    let onclick = Callback::from(|_| {
        // Handle click event
    });

    html! {
        <table class="">
            <tr>
                <td class="w-1/2">
                    <button onclick={onclick} class="w-full bg-brand-primary text-brand-text rounded">
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
                        <div class="text-left p-3 text-sm">
                            {"Game with a friend on the same device."}
                        </div>
                    </button>
                </td>
                <td class="w-1/2">
                    <button disabled={true} class="w-full bg-brand-secondary text-brand-text rounded">
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
                        <div class="text-left p-3 text-sm">
                            {"Game with another player online."}
                        </div>
                    </button>
                </td>
            </tr>
        </table>
    }
}
