use serde::{Serialize, Deserialize, Serializer, Deserializer};
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
    Ok(app)
}

/*
    pub(crate) fn save(&self) -> Result<(), ConfigError> {
        let toml_pretty = toml::to_string_pretty(self)
            .change_context(ConfigError::TomlError)?
            .into_bytes();
        // The TMS_CONFIG_FILE envvar should be set, either by the user or when the config is
        // loaded. However, there is a possibility it becomes unset between loading and saving
        // the config. In this case, it will fall back to the platform-specific config folder, and
        // if that can't be found then it's good old ~/.config
        let path = match env::var("TMS_CONFIG_FILE") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                if let Some(config_path) = dirs::config_dir() {
                    config_path.as_path().join("tms/config.toml")
                } else if let Some(home_path) = dirs::home_dir() {
                    home_path.as_path().join(".config/tms/config.toml")
                } else {
                    return Err(ConfigError::LoadError)
                        .attach_printable("Could not find a valid location to write config file (both home and config dirs cannot be found)")
                        .attach(Suggestion("Try specifying a config file with the TMS_CONFIG_FILE environment variable."));
                }
            }
        };
        let parent = path
            .parent()
            .ok_or(ConfigError::FileWriteError)
            .attach_printable(format!(
                "Unable to determine parent directory of specified tms config file: {}",
                path.to_str()
                    .unwrap_or("(path could not be converted to string)")
            ))?;
        std::fs::create_dir_all(parent)
            .change_context(ConfigError::FileWriteError)
            .attach_printable("Unable to create tms config folder")?;
        let mut file = std::fs::File::create(path).change_context(ConfigError::FileWriteError)?;
        file.write_all(&toml_pretty)
            .change_context(ConfigError::FileWriteError)?;
        Ok(())
    }
*/
