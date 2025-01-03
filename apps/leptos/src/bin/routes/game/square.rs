use leptos::*;

#[component]
pub fn Square(
    children: Children,
    #[prop(into)] size: Signal<u16>,
    #[prop(into)] name: Signal<String>,
    #[prop(into)] id: Signal<String>,
    #[prop(into)] color: Signal<String>,
    #[prop(into)] on_select: Signal<impl Fn() + 'static>,
) -> impl IntoView {
    let classname = move || format!("square {}", color.get());
    let style = move || format!("width: {}px;height:{}px;", size.get(), size.get());
    view! {
        <div
            id=id
            data-square=name
            class=classname
            style=style
            on:click=move |_| on_select.with(|on_select| on_select())
        >
            {children()}
        </div>
    }
}
