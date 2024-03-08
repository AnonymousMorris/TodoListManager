use core::panic;
use std::fs;
use home;

use crate::app::App;
use crate::app::*;

pub fn save(app: &App) {
    let home = match home::home_dir() {
            Some(dir) => dir,
            None => panic!("error getting home directory"),
        };
    let app_path = home.join(".todo-list-manager");
    if !app_path.exists() {
        match fs::create_dir(&app_path){
            Err(e) => panic!("failed to create dir with error: {}", e),
            Ok(_) => ()
        }
        print!("created dir: ~/.todo-list-manager");
    }
    let serialize = serde_json::to_string(&app).unwrap();
    let file_path = app_path.join("todos.json");
    match fs::write(file_path, serialize) {
        Err(e) => panic!("write failed with error: {}", e),
        Ok(_) => ()
    }
}
pub fn retrieve() -> std::result::Result<App, std::io::Error> {
    let home = match home::home_dir() {
            Some(dir) => dir,
            None => panic!("error getting home directory"),
        };
    let app_path = home.join(".todo-list-manager");
    if !app_path.exists() {
        match fs::create_dir(&app_path){
            Err(e) => return Err(e),
            Ok(_) => return Ok(App::new()),
        }
    }
    let file_path = app_path.join("todos.json");
    let todos = match fs::read_to_string(file_path) {
        Err(e) => return Err(e),
        Ok(result) => result,
    };
    let mut app: App = match serde_json::from_str(&todos) {
        Ok(data) => data,
        Err(_) => App::new(),
    };
    app.mode = Mode::Normal;
    app.line_num = None;
    app.visual_begin = None;
    for todolist in &mut app.todolists {
        for todo in &mut todolist.todos {
            todo.selected = false;
            todo.editing = false;
        }
    }
    app.command.value = String::new();
    Ok(app)
}
