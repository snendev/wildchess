use leptos::*;
use leptos_router::*;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <A href="/play">Go Online</A>
            <A href="/fake-board">Show Fake Board</A>
        </div>
    }
}
