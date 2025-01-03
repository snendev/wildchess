use leptos::*;

#[component]
pub fn Grid<F, V>(
    #[prop(into)] rows: Signal<u16>,
    #[prop(into)] columns: Signal<u16>,
    children: F,
) -> impl IntoView
where
    F: Fn(u16, u16) -> V + Clone + 'static,
    V: IntoView + 'static,
{
    view! {
        <For
            each=move || 0..rows.get()
            key=|row| *row
            children=move |row| {
                let children = children.clone();
                view! {
                    <div class="row">
                        <For
                            each=move || 0..columns.get()
                            key=|column| *column
                            children=move|column| {
                                children(row, column)
                            }
                        />
                    </div>
                }
            }
        />
    }
}
