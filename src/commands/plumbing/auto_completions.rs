use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::app::App;
use crate::domain::TodoList;

impl TodoList {
    pub fn auto_completions(shell: Shell) {
        let mut cmd = App::command();
        generate(shell, &mut cmd, "todo", &mut std::io::stdout());
    }
}
