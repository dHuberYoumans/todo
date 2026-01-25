use microxdg::Xdg;
use std::path::PathBuf;

#[derive(Clone)]
pub struct UserPaths {
    pub home: PathBuf,
    pub config: Option<PathBuf>,
    pub todo_config: Option<PathBuf>,
}

impl Default for UserPaths {
    fn default() -> Self {
        UserPaths::new()
    }
}

impl UserPaths {
    pub fn new() -> Self {
        let xdg = Xdg::new().expect("✘ Could not reslove XDG directories");
        let home = xdg.home().to_path_buf();
        let config = xdg.config().ok();
        let todo_config = xdg.config().map(|conf| conf.join("todo/todo.config")).ok();
        Self {
            home,
            config,
            todo_config,
        }
    }
}
