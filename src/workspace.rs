use crate::environment::Credential;
use crate::git::Git;
use crate::log::Log;
use std::io;

pub struct Workspace {
    git: Git,
}

impl Workspace {
    pub fn new() -> Self {
        let git = Git::new();

        Self { git }
    }

    pub fn create_config(&mut self, credentials: &Credential) -> io::Result<()> {
        self.git.set_user_name(&credentials.username);
        self.git.set_user_email(&credentials.email);

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

        if current_user.contains("davejs136") {
            Log::status("Personal");
        } else {
            Log::status("Work");
        }

        Ok(())
    }
}
