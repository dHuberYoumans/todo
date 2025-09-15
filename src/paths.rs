use microxdg::Xdg;
use std::path::PathBuf;

pub struct UserPaths {
    pub home: PathBuf,
    pub config: Option<PathBuf>,
}

impl UserPaths {
    pub fn new() -> Self{
        let xdg = Xdg::new().expect("✘ Could not reslove XDG directories");
        let home = xdg.home().to_path_buf();
        let config = xdg.config().map(|conf| conf.join("todo/todo.config")).ok();
        Self { home, config } 
    }
}
