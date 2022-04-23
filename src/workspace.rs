use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write as _;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time;

use crate::log::Log;

static SSH_PATH: &str = "/home/dave/.ssh";
static FILENAME: &str = "config";

const THREADS: u32 = 2;

#[derive(Debug)]
pub enum Env {
    Personal,
    Work,
}

pub struct Workspace {}

impl Workspace {
    pub fn create_config(env: Env) -> io::Result<()> {
        match env {
            Env::Personal => {
                let mut processes = vec![];
                // TODO: Extract info to json
                let email = "davejs136@gmail.com";
                let username = "Dave136";
                let config_file = Workspace::ssh_config("Personal", "id_rsa");
                Log::info("Write config content...");
                Log::info("Setting global config for git ...");
                Workspace::write_content(&config_file)?;

                for _ in 0..THREADS {
                    processes.push(thread::spawn(move || {
                        let (email_param, user_param) = Workspace::github_config(email, username);
                        Workspace::exec_git_command(email_param);
                        Workspace::exec_git_command(user_param);
                        thread::sleep(time::Duration::from_secs(1));
                    }));
                }

                for process in processes {
                    let _ = process.join();
                }

                let user_message = format!("User  -> {}", &username);
                let email_message = format!("Email -> {}", &email);

                let user_str = &user_message[..];
                let email_str = &email_message[..];

                Log::success(user_str);
                Log::success(email_str);

                Ok(())
            }
            Env::Work => {
                let mut processes = vec![];
                // TODO: Extract info to json
                let email = "darenas@aluxion.com";
                let username = "Davee136";
                let config_file = Workspace::ssh_config("Work", "id_rsa_work");
                Log::info("Writing configuration content...");
                Workspace::write_content(&config_file)?;

                Log::info("Setting the global configuration for git ...");
                for _ in 0..THREADS {
                    processes.push(thread::spawn(move || {
                        let (email_param, user_param) = Workspace::github_config(email, username);
                        Workspace::exec_git_command(email_param);
                        Workspace::exec_git_command(user_param);
                        thread::sleep(time::Duration::from_secs(1));
                    }));
                }

                for process in processes {
                    let _ = process.join();
                }

                let user_message = format!("User  -> {}", &username);
                let email_message = format!("Email -> {}", &email);

                let user_str = &user_message[..];
                let email_str = &email_message[..];

                Log::success(user_str);
                Log::success(email_str);
                Ok(())
            }
        }
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

    fn write_content(content: &str) -> io::Result<()> {
        let path = Path::new(SSH_PATH).join(FILENAME);

        // open file in write-only mode, returns io::Result<File>
        let mut file = File::create(&path)?;

        // Write string into file, returns io::Result<()>
        file.write_all(content.as_bytes())?;
        Log::success("Config created successfully");
        Ok(())
    }

    fn read() -> io::Result<String> {
        let path = Path::new(SSH_PATH);
        let config_path = path.join("config");

        let mut file = File::open(&config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    }

    fn ssh_config(title: &str, rsa: &str) -> String {
        format!("# {} account \n\tHostName github.com\n\tUser git\n\tIdentityFile ~/.ssh/{}\n",
            title, rsa)
    }

    fn github_config(email: &str, username: &str) -> (String, String) {
        let email_config = format!("git config --global user.email \"{}\"", email);
        let user_config = format!("git config --global user.name \"{}\"", username);

        (email_config, user_config)
    }

    fn exec_git_command(args: String) {
        Command::new("zsh")
            .arg("-c")
            .arg(args)
            .output()
            .expect("There was a problem executing git command");
    }
}
