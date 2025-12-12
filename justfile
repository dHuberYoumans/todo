# Build & install the CLI into ~/.cargo/bin
install: build-release install-completions 
	cp target/release/todo ~/.cargo/bin

build-release:
	cargo build --release -p todo-cli

# Query CLI for paths to local database and config file
paths:
	todo show-paths

# Clean old install
clean:
	todo clean-data

# Initialize new list
init:
    todo init

# reinstall with default settings
reset: clean install init

# Install shell completions for the current shell
install-completions: 
    #!/usr/bin/env bash
    set -e

    SHELL_NAME="$(basename "$SHELL")"
    echo "Detected shell: $SHELL_NAME"

    case "$SHELL_NAME" in
        zsh)
            DEST="$HOME/.zsh/completions"
            mkdir -p "$DEST"
            cp ./todo-cli/completions/_todo "$DEST/_todo"
            echo -e "\033[32mInstalled zsh completion to $DEST/_todo\033[0m"
            echo -e "\033[33mMake sure ~/.zshrc contains:\033[0m"
            echo -e "\033[33m  fpath=(~/.zsh/completions \$fpath)\033[0m"
            echo -e "\033[33m  autoload -Uz compinit && compinit\033[0m"
            ;;
        bash)
            DEST="$HOME/.bash_completion.d"
            mkdir -p "$DEST"
            cp ./todo-cli/completions/todo.bash "$DEST/todo"
            echo -e "\033[32mInstalled bash completion to $DEST/_todo\033[0m"
            echo -e "\033[33mYou may need to add this to ~/.bashrc:\033[0m"
            echo -e "\033[33m  for f in ~/.bash_completion.d/*; do source \"\$f\"; done\033[0m"
            ;;
        fish)
            DEST="$HOME/.config/fish/completions"
            mkdir -p "$DEST"
            cp ./todo-cli/completions/todo.fish "$DEST/todo.fish"
            echo -e "\033[32mInstalled fish completion to $DEST/_todo\033[0m"
            echo -e "\033[33mRestart fish or open a new terminal\033[0m"
            ;;
        *)
            echo "Unsupported shell: $SHELL_NAME"
            echo "Supported: zsh, bash, fish"
            exit 1
            ;;
    esac


# bump version
bump-version *ARGS:
    ./.bump-to.sh {{ARGS}}
