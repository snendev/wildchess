use leptos::*;
use leptos_router::*;
mod routes;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(move || {
        view! {
            <Router>
                <nav>
                // TODO navbar
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=routes::Index />
                        <Route path="/play" view=routes::Game />
                        <Route path="/fake-board" view=routes::FakeBoard />
                        <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>
        }
    })
}
