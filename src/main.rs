use gui::Gui;
use iced::{Sandbox, Settings};

mod gui;
mod sql;

fn main() {
    Gui::run(Settings::default()).unwrap();
}
