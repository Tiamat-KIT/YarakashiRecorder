mod app;
mod component;
mod structs;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::ERROR).expect("failed to init logger");
    launch(App);
}
