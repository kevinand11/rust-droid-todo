use druid::{
    widget::{Button, Checkbox, Controller, Flex, Label, List, Padding, TextBox, ZStack},
    Code, Color, Env, Event, EventCtx, Menu, MenuItem, Point, UnitPoint, Widget, WidgetExt,
};

use crate::{
    data::{TodoItem, TodoState},
    save::Saver,
};

pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        .with_flex_child(
            TextBox::new()
                .with_placeholder("Start Typing")
                .lens(TodoState::new_text)
                .expand_width()
                .controller(Enter {}),
            1.,
        )
        .with_child(Button::new("->").on_click(|_, state: &mut TodoState, _| update_state(state)))
        .with_child(Saver {});

    let todos = List::new(|| {
        let bg = Color::rgba(0., 0., 0., 50.);
        let todo = Flex::row()
            .with_child(Label::new(|todo: &TodoItem, _: &Env| todo.text.clone()))
            .with_child(Padding::new(
                (5., 0.),
                Checkbox::new("").lens(TodoItem::checked),
            ))
            .with_flex_spacer(0.1)
            .with_child(Button::new("...").on_click(
                |ctx: &mut EventCtx, item: &mut TodoItem, _env| {
                    let item = item.clone();
                    let menu: Menu<TodoState> =
                        Menu::empty().entry(MenuItem::new("Remove").on_activate(
                            move |_ctx, state: &mut TodoState, _| {
                                let index = state.todos.index_of(&item).unwrap();
                                state.todos.remove(index);
                            },
                        ));
                    ctx.show_context_menu(menu, Point::new(0., 0.))
                },
            ));
        Padding::new(5., todo).background(bg)
    })
    .lens(TodoState::todos)
    .scroll()
    .vertical();

    let clear_complete = Button::new("Clear Complete").on_click(|_, state: &mut TodoState, _| {
        state.todos.retain(|item| !item.checked);
    });

    ZStack::new(
        Flex::column()
            .with_child(header)
            .with_flex_child(todos.expand_width(), 1.),
    )
    .with_aligned_child(Padding::new(5., clear_complete), UnitPoint::BOTTOM_RIGHT)
}

struct Enter;

impl<W: Widget<TodoState>> Controller<TodoState, W> for Enter {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut TodoState,
        env: &Env,
    ) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {
                update_state(data)
            }
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

fn update_state(state: &mut TodoState) {
    if state.new_text.trim() != "" {
        let text = state.new_text.clone();
        state.new_text = "".to_string();
        state.todos.push_back(TodoItem {
            checked: false,
            text,
        })
    }
}
