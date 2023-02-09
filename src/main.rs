use data::TodoState;
use druid::{WindowDesc, AppLauncher};
use im::Vector;
use save::read_stored;

use crate::ui::ui_builder;

mod ui;
mod data;
mod save;

fn main() {
    let window = WindowDesc::new(ui_builder())
        .title("My Todo App")
        .window_size((400., 400.));

    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(window)
        .launch(default_state)
        .expect("Failed to launch");
}
