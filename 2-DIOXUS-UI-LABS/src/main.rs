mod app;
mod components;
mod routes;
mod routing;
mod shared;

use app::App;

fn main() {
    dioxus::launch(App);
}
