use core::panic;
use std::fs;
use std::path::PathBuf;
use home;

use crate::app::App;

const CONFIG_RELATIVE_PATH: &str = ".config/todolist-manager";
const CONFIG_FILE_NAME: &str = "todos.json";

fn config_dir() -> PathBuf {
    let home_dir = match home::home_dir() {
            Some(dir) => dir,
            None => panic!("error getting home directory"),
        };
    let config_path = home_dir.join(CONFIG_RELATIVE_PATH);
    return config_path;
}

pub fn save(app: &App) {
    let config_dir = config_dir();
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    if !config_dir.exists() {
        match fs::create_dir(&config_dir){
            Err(e) => panic!("failed to create dir with error: {}", e),
            Ok(_) => ()
        }
        print!("created dir: {}", config_path.display());
    }
    let serialize = serde_json::to_string(&app).unwrap();
    match fs::write(config_path, serialize) {
        Err(e) => panic!("write failed with error: {}", e),
        Ok(_) => ()
    }
}

pub fn retrieve() -> std::result::Result<App, std::io::Error> {
    let config_dir = config_dir();
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    if !config_dir.exists() {
        match fs::create_dir(&config_dir){
            Err(e) => return Err(e),
            Ok(_) => return Ok(App::new()),
        }
    }
    let todos = match fs::read_to_string(config_path) {
        Err(e) => return Err(e),
        Ok(result) => result,
    };
    let app: App = match serde_json::from_str(&todos) {
        Ok(data) => data,
        Err(_) => App::new(),
    };
    Ok(app)
}
