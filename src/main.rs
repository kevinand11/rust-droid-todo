use data::TodoState;
use druid::{
    theme::{BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR},
    AppLauncher, Color, WindowDesc,
};
use im::Vector;
use save::read_stored;

use crate::ui::ui_builder;

mod data;
mod save;
mod ui;

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
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 100, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(0, 0, 0, 100));
        })
        .launch(default_state)
        .expect("Failed to launch");
}
