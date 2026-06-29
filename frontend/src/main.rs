mod app;

use app::App;

fn main() {
    leptos::mount_to_body(|| leptos::view! { <App /> });
}
