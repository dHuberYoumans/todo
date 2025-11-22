# Build & install the CLI into ~/.cargo/bin
install: 
	cargo build --release
	cp target/release/todo ~/.cargo/bin

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
