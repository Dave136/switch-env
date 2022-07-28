use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub name: String,
    pub credentials: Credential,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub list: Vec<Detail>,
}

impl Environment {
    /// Create a new instance of `Environment` and loads the config file
    ///
    /// If the file not exists, it will be created as empty
    pub fn new() -> Self {
        let cwd = Environment::config_path();
        let mut file = File::open(cwd).unwrap_or_else(|_| {
            let dir = Environment::config_path();
            Environment::create_initial_config();
            File::open(dir).unwrap()
        });
        let mut content = String::new();

        match file.read_to_string(&mut content) {
            Ok(_) => {
                let config: Environment = serde_json::from_str(&content).unwrap();

                config
            }
            Err(_) => {
                Environment::create_initial_config();
                Environment { list: Vec::new() }
            }
        }
    }

    /// Returns the config file path
    ///
    /// `$HOME/.switch-env/env.json`
    fn config_path() -> PathBuf {
        let mut cwd = home_dir().unwrap();
        cwd.push(".switch-env");
        cwd.push("env.json");

        cwd
    }

    /// Create an initial env.json
    ///
    fn create_initial_config() {
        let cwd = Environment::config_path();
        let path_exists = Path::new(&cwd).exists();

        if !path_exists {
            let environment = Environment { list: Vec::new() };
            let environment_json = serde_json::to_string(&environment).unwrap();
            let mut file = File::create(&cwd).unwrap();

            file.write_all(environment_json.as_bytes()).unwrap();
        }
    }
}
