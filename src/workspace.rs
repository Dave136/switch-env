use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write as _;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time;

use crate::git::Git;
use crate::log::Log;

static SSH_PATH: &str = "/.ssh";
static FILENAME: &str = "config";

const THREADS: u32 = 2;

#[derive(Debug)]
pub enum Env {
    Personal,
    Work,
}

pub struct Workspace {
    git: Git,
}

impl Workspace {
    pub fn new() -> Self {
        let git = Git::new();

        Self { git }
    }

    pub fn create_config(&mut self, env: Env) -> io::Result<()> {
        match env {
            Env::Personal => {
                // TODO: Extract info to json
                let email = "davejs136@gmail.com";
                let username = "Dave136";
                // let config_file = Workspace::ssh_config("Personal", "id_rsa");
                // Log::info("Write config content...");
                // Log::info("Setting global config for git ...");
                // Workspace::write_content(&config_file)?;

                self.git.set_user_name(username);
                self.git.set_user_email(email);
            }
            Env::Work => {
                // TODO: Extract info to json
                let email = "darenas@aluxion.com";
                let username = "Davee136";
                // let config_file = Workspace::ssh_config("Work", "id_rsa_work");

                // Log::info("Writing configuration content...");
                // Workspace::write_content(&config_file)?;

                Log::info("Setting the global configuration for git ...");
                self.git.set_user_name(username);
                self.git.set_user_email(email);
            }
        }

        Log::success(&format!("User  -> {}", self.git.get_user_name()));
        Log::success(&format!("Email -> {}", self.git.get_user_email()));

        Ok(())
    }

    pub fn show_status() -> io::Result<()> {
        let file = Workspace::read()?;

        if file.contains("Personal") {
            Log::status("Personal");
        } else {
            Log::status("Work");
        }

        Ok(())
    }

    // fn write_content(content: &str) -> io::Result<()> {
    //     let path = Path::new(SSH_PATH).join(FILENAME);

    //     // open file in write-only mode, returns io::Result<File>
    //     let mut file = File::create(&path)?;

    //     // Write string into file, returns io::Result<()>
    //     file.write_all(content.as_bytes())?;
    //     Log::success("Config created successfully");
    //     Ok(())
    // }

    fn read() -> io::Result<String> {
        let path = Path::new(SSH_PATH);
        let config_path = path.join("config");

        let mut file = File::open(&config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    }

    // fn ssh_config(title: &str, rsa: &str) -> String {
    //     format!(
    //         "# {} account \n\tHostName github.com\n\tUser git\n\tIdentityFile ~/.ssh/{}\n",
    //         title, rsa
    //     )
    // }
}
