# Install 
install: build-release install-completions

# Build & install the CLI into ~/.cargo/bin
build-release: 
	cargo build --release
	cp target/release/todo ~/.local/bin

# Install auto-completions
install-completions:
    shell=${SHELL##*/} && \
    case "$shell" in \
      bash|zsh|fish) ;; \
      *) echo "Unsupported shell: $shell" >&2; exit 1 ;; \
    esac && \
    todo completions install "$shell"

# reinstall with default settings
reset: clean install init

# Clean old install
clean:
	todo clean-data

# Initialize
init:
    todo init

# Query CLI for paths to local database and config file
paths:
	todo show-paths

# bump version
bump-version *ARGS:
    ./.bump-to.sh {{ARGS}}
