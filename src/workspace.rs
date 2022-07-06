use crate::git::Git;
use crate::log::Log;
use std::io;

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

                self.git.set_user_name(username);
                self.git.set_user_email(email);
            }
            Env::Work => {
                // TODO: Extract info to json
                let email = "darenas@aluxion.com";
                let username = "Davee136";

                Log::info("Setting the global configuration for git ...");
                self.git.set_user_name(username);
                self.git.set_user_email(email);
            }
        }

        Log::success_custom(
            &format!("User  => "),
            &format!("{}", self.git.get_user_name()),
        );
        Log::success_custom(
            &format!("Email => "),
            &format!("{}", self.git.get_user_email()),
        );

        Ok(())
    }

    pub fn show_status(&mut self) -> io::Result<()> {
        let current_user = self.git.get_user_email();
        // let file = Workspace::read()?;

        if current_user.contains("davejs136") {
            Log::status("Personal");
        } else {
            Log::status("Work");
        }

        Ok(())
    }
}
