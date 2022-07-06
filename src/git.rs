use git2::Config;
use home::home_dir;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
// use std::str::FromStr;

const USER_EMAIL: &str = "user.email";
const USER_NAME: &str = "user.name";

pub struct Git {
    config: Config,
    config_path: PathBuf,
    user_name: String,
    user_email: String,
}

impl Git {
    /// Creates a new instance of `Git` and loads the global config for Git.
    ///
    /// As a side effect, a backup of the global config is created using
    /// `Git::backup`.
    pub fn new() -> Self {
        let mut config_path = home_dir().unwrap();
        config_path.push(".gitconfig");
        Git::backup(&config_path);

        let config = Config::open(&config_path).unwrap();
        let user_name = config.get_string(USER_NAME).unwrap().to_string();
        let user_email = config.get_string(USER_EMAIL).unwrap().to_string();

        Self {
            config,
            config_path,
            user_name,
            user_email,
        }
    }

    /// Retrieves the global config user.name and stores it as part of the
    /// `Git` config state
    #[inline]
    pub fn get_user_name(&mut self) -> String {
        let user_name = self.config.get_string(USER_NAME).unwrap().to_string();

        self.user_name = user_name.clone();
        user_name
    }

    /// Retrieves the global config user.email and stores it as part of the
    /// `Git` config state
    #[inline]
    pub fn get_user_email(&mut self) -> String {
        let user_email = self.config.get_string(USER_EMAIL).unwrap().to_string();

        self.user_email = user_email.clone();
        user_email
    }

    /// Sets the global config user.name and stores it as part of the `Git`
    /// config state
    #[inline]
    pub fn set_user_name(&mut self, name: &str) {
        self.config.set_str("user.name", name).unwrap();
        self.user_name = name.to_string();
    }

    /// Sets the global config user.email and stores it as part of the `Git`
    /// config state
    #[inline]
    pub fn set_user_email(&mut self, email: &str) {
        self.config.set_str("user.email", email).unwrap();
        self.user_email = email.to_string();
    }

    /// Copies the current `$HOME/.gitconfig` file into
    /// `$PWD/tmp/gitconfig_backup`
    ///
    /// This file then can be used to reset changes applied to the `.gitconfig`
    /// file when first running git operations.
    fn backup<P>(config_path: P)
    where
        P: AsRef<Path>,
    {
        let mut cwd = home_dir().unwrap();
        cwd.push(".switch-env");
        cwd.push("tmp");
        cwd.push("gitconfig_backup");

        let path_exists = Path::new(&cwd).exists();

        // If not exist, we create the directory to avoid errors
        if !path_exists {
            let mut dir = home_dir().unwrap();
            dir.push(".switch-env");
            dir.push("tmp");
            fs::create_dir_all(&dir).unwrap();
        }

        fs::copy(&config_path, &cwd).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Git;

    #[test]
    fn sets_git_global_config() {
        let mut git = Git::new();
        let current_user_name = git.get_user_name();
        let current_user_email = git.get_user_email();

        git.set_user_name("Hello World");
        git.set_user_email("hello@world.com");

        assert_eq!(git.user_name, "Hello World");
        assert_eq!(git.user_email, "hello@world.com");

        git.set_user_name(&current_user_name);
        git.set_user_email(&current_user_email);

        assert_eq!(git.get_user_name(), current_user_name);
        assert_eq!(git.get_user_email(), current_user_email);
    }
}
