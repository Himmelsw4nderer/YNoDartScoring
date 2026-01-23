use yew::prelude::*;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    html! {
        <div class="home">
            <header>
                <h1>{"Welcome Home"}</h1>
            </header>
            <main>
                <section>
                    <h2>{"Get Started"}</h2>
                    <p>{"This is your home view. Add your content here."}</p>
                </section>
            </main>
        </div>
    }
}
