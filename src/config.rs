





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
