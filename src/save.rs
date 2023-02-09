use std::{fs, path::Path};

use directories::BaseDirs;
use druid::{Size, Widget};
use serde::{Deserialize, Serialize};

use crate::data::{TodoItem, TodoState};

pub struct Saver;

impl Widget<TodoState> for Saver {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut TodoState,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &TodoState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        _env: &druid::Env,
    ) {
        if data.todos != old_data.todos {
            let config = get_config_path();
            let config_path = Path::new(&config);
            let tasks = TaskData {
                tasks: data.todos.clone().into_iter().collect(),
            };
            fs::write(config_path, serde_json::to_string(&tasks).unwrap())
                .expect("Config path does not exist");
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &TodoState,
        _env: &druid::Env,
    ) -> druid::Size {
        Size {
            width: 0.,
            height: 0.,
        }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &TodoState, _env: &druid::Env) {}
}

#[derive(Serialize, Deserialize, Default)]
pub struct TaskData {
    pub tasks: Vec<TodoItem>,
}

pub fn read_stored() -> TaskData {
    let config = get_config_path();
    let config_path = Path::new(&config);
    let data = match fs::read_to_string(config_path) {
        Ok(a) => a,
        Err(_) => return TaskData::default(),
    };
    match serde_json::from_str(&data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Save data is corrupted\nError {}", e);
            TaskData::default()
        }
    }
}

fn get_config_path() -> String {
    let filename = "druid-todos.json";
    if let Some(base_dirs) = BaseDirs::new() {
        return format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), filename);
    }
    format!("./{}", filename)
}
