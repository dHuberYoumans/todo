# TODO

[![GitHub release](https://img.shields.io/github/v/release/dHuberYoumans/todo)](https://github.com/dHuberYoumans/todo/releases/latest)
[![GitHub tag](https://img.shields.io/github/v/tag/dHuberYoumans/todo)](https://github.com/dHuberYoumans/todo/tags)
[![Last commit](https://img.shields.io/github/last-commit/dHuberYoumans/todo)](https://github.com/dHuberYoumans/todo/commits/main)
[![CI](https://github.com/dHuberYoumans/todo/actions/workflows/ci.yml/badge.svg)](https://github.com/dHuberYoumans/todo/actions/workflows/ci.yml)

A rusty todo list CLI.

## Installations

Supported platforms:
- macOS (Intel, Apple Silicon)
- Linux (x86_64, ARM64)

### Using curl | sh
The easiest way to install _todo_ is to use the `install.sh` script:
```console
curl -fsSL https://raw.githubusercontent.com/dHuberYoumans/todo/main/install.sh | sh
```
It will install the _todo_ CLI (into `$HOME/.local/bin/`) alongside with its auto-completions. You may have to restart your terminal or open a new session afterwards. 

**Remark:** see [Shell auto-completion](#shell-auto-completion) for how to setup autocompletions correctly.

In order to install a specific version, say `x.y.z` of the CLI, use
```console
$ curl -fsSL https://raw.githubusercontent.com/dHuberYoumans/todo/main/install.sh | VERSION=x.y.z sh
```

Use 
```console
$ todo -V|--version 
```
to see the current installed version.

### Build from source
Prerequisites: `rustrc`, `cargo` and `just` installed.

After having cloned the repo you can simply run
```console
just install
```
This will build the release version from the source code using `cargo build --release`, copy the binary into `~/.local/bin` and install the auto-completions.

## First steps
When running _todo_ for the first time, it is recommended to run `todo init`.
It will create the folder `~/.todo/` and two files

1. `~/.todo/.env`  
2. `~/.todo/todo.db`

The dotenv file contains a line `TODO_DB=todo.db` and is used to load the todo list `~/.todo/todo.db`. 
_todo_ allows you to create and load several lists via `todo new-list [name]` and `todo load [name]` (see the help menu for more details).
The loading is done by setting the dotenv variable `TODO_DB=name.db`. 


## todo help
To see an exhaustive list of all commands, please consult the help menu:
```console
$ todo --help

A simple todo cli to help you get things done from the comfort of your terminal

Usage: todo [COMMAND]

Commands:
  init         Initialize the cli in CWD
  config       Open config
  new-list     Create a new todo list
  delete-list  Delete a todo list
  load         Load a todo list
  whoami       Print the name of the todo list in use to stdout
  add          Add a task
  list         Print the current todo list
  show         Show metadata of a task
  close        Mark a task as completed
  close-all    Mark all tasks as completed
  open         Open a task
  delete       Delete a task
  delete-all   Delete all tasks in the current todo list
  grep         Search a pattern inside todos
  reword       Reword a task
  rnd          Get a random todo among those with prio = RNG
  update       Update the fields of an item
  clear        Clear due, prio or the tag column
  upgrade      Upgrade the CLI
  show-paths   Show user paths
  clean-data   Clean data
  completions  Generates auto-completions
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```
In addition, each command has its own help flag
```console
$ todo init --help

Initialize the cli in CWD

Usage: todo init

Options:
  -h, --help  Print help
```

## Shell auto-completion
To install auto-completions use
```console
$ todo completions install <shell>
```
This will generate and install the auto-compleiton files for the CLI into the following standard paths:
- zsh: `~/.zsh/completions/_todo`
- bash: `~/.local/share/bash-completion/completions/todo`
- fish: `~/.config/fish/completions/todo.fish`

If you have your custom completion search path or directory, you can use
```console
$ todo completions generate <shell> > <completion search path>
```
The command `todo completions generate` generates the completions function file and prints it to stdout.

Finally, restart your terminal to activate the completions.

**Remark:** The auto-completions files are installed into custom folders which might have to be added to the corresponding shell configuration files (e.g. .zshrc, .bashrc). 

- zsh: add `fpath=(~/.zsh/completions $fpath)` to your .zshrc before calling `compinit`
- bash: if not done already, install `bash-completion` and add the following to your .bashrc:
  on Linux
  ```bash
  # Load bash-completion
  source usr/share/bash-completion/bash_completion.sh
  export BASH_COMPLETION_USER_DIR="$HOME/.local/share/bash-completion"
  ```
  on macOS (assuming `bash-completion` was installed via homebrew: `brew install bash-completion@2`)
  ```bash
  # Load bash-completion
  source "$(brew --prefix)/etc/profile.d/bash_completion.sh"
  export BASH_COMPLETION_USER_DIR="$HOME/.local/share/bash-completion"
  ```
- fish: works on the nose ><>

## Up- and downgrading
_todo_ offers a command to upgrade (or downgrade) to a specific release version. 
```
$ todo upgrade [--version vX.Y.Z]
```
If no version is provided, it will upgrade to the latest available release version on GitHub.

## Basic Usage
The usage is fairly standard
```console
$ todo [cmd] [options] [args]
```

### Printing the todo list
The most basic command is the `list` command which prints the todo list to stdout.
```console
$ todo list --help

Print the current todo list

Usage: todo list [OPTIONS] [ARG]

Arguments:
  [ARG]

Options:
      --filter <FILTER>  Filter tasks [possible values: none, do, done]
  -s, --sort <SORT>      Sort tasks
      --collection       Show collection
      --tags             Display available tags
  -h, --help             Print help

```
Since it is used so often, it is the default command that is the following two are equivalent
```console
$ todo 
$ todo list
```
In addition, 
```
$ todo list #tag <--filter FILTER>
```
lists only those todos with the corresponding tag, while 
```console
$ todo list @due <--filter FILTER>
```
lists only those todos with the corresponding due date.   

### Add a todo
The next most common command is `add`. It adds a new todo to the current list.
```console
$ todo add --help

Add a task

Usage: todo add [OPTIONS]

Options:
  -m, --task <TASK>  Task description
  -p, --prio <PRIO>  Priority [possible values: p1, p2, p3, empty]
  -d, --due <DUE>    Due date
  -t, --tag <TAG>    Tag
  -h, --help         Print help
```
When we omit the `-m` option, then the standard editor is opened allowing us to write longer and more detailed task descriptions. The CLI follows `git` in that it treats the first line as the title and the rest as the body. When printing the todo list to stdout, only the title is displayed.

### Close/Open a todo
The `close` and `open` commands might be self-explanatory: they open or close the task with the given id.

**Remark:** Each task has a unique ID (uuid). When passing an ID as an argument to a command, we don't have to specify the full ID but only as many digits as we need to uniquely identify the task. For example, say we have two tasks with ID `abcd` and `abef`. Then passing `ab` will give an error `Ambiguous prefix`. This error indicates that the prefix `ab` doesn't resolve to a unique ID. However, `abc` would not cause this error.

### Update a todo
If you need to update the state - usually either one or more of priority, due date or tag - of a todo, you can use the `update` command.
```console
$ todo update --help

Update the fields of an item

Usage: todo update [OPTIONS] [IDS]...

Arguments:
  [IDS]...  

Options:
  -d, --due <DUE>        Update the due date
  -p, --prio <PRIO>      Update the priority [possible values: p1, p2, p3, empty]
  -s, --status <STATUS>  Update the status [possible values: closed, open]
  -t, --tag <TAG>        Update the tag
  -h, --help             Print help
```

### Show
Next, we want to point out the `show` command. It is similar to `git`, the `show` command displays the todo task with metadata. In particular it shows the body of the task. For example
```console
$ todo show 59

Id: 59bfa408-42d8-4260-b554-4e48c0ecc285
Created at: 2025-11-29
Last updated at: Today
Due by: 2025-11-28
Priority: P1
Status: open
Tag: #friends

Once upon a time...

There was a message to the world: hello!
```
The show command supports minimal markdown syntax:
- italic: wrap the word in underscores → \_italic\_ renders as _italic_
- bold: wrap the word in single star → \*bold\* renders as **bold**
- bulleted list: use a hyphen → - foo renders as • foo
- checkboxes: use - [ ] or - [x] which renders as □ and ■ respectively
- quotes: use > quote 
- links: use the standard notation \[text\]\(url\)

### Search
The CLI also supports a basic search to find todos that contain a pattern
```console
$ todo grep <option> "<pattern>"
```
The command will comb each todo in the todo list for "pattern" and prints a table containing only those todos which yielded a match.
At the current state the search is still quite basic - the only option it supports so far is the `-i|--ignore` flag which enables case-insensitive search.

### Rnd
If you have troubles deciding on which todo to start next, try to update a bunch of todos' priority to `RND` and run
```console
$ todo rnd
```
The CLI will give you a random todo from among those with priority `RND` to start.

## Configuration
The CLI allows a configuration file which is automatically generated in `~/.config/todo/todo.config` when running `todo init`. 
The default configuration file is of the following form
```toml
[database]
todo_db = "/User/name/.todo/todo.db"

[style]
id_length = 6
due_date_display_format = "%x" # chrono strftime-style
due_date_input_format = "DMY" # MDY | ISO
show_due = true
show_tag = true
sort_by = "prio"  # prio | due | tag
table = "modern_rounded" # ascii | ascii_rounded | modern |  modern_rounded | markdown"
```
The `database` section contains the field `todo_db` which is the path to the sqlite database.
The `style` section cintains several fields concerning the style of the todo list (or the table) when printed to stdout:
- `id_length` is a number and controlls the lenght of the id-prefix displayed in the id column
- `due_date_format` is a string and controlls the format of the due date. It follows the strftime-style. See [chrono::format::strftime](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) for more details
- `show_due` is a boolean value that controlls the visibility of the `due` column
- `show_tag` is a boolean value that controlls the visibility of the `tag` column
- `sort_by` is one of the following strings: "prio", "due" or "tag". It controlls the default sorting of the table (by prio, due or tag)
- `table` is a string (either of  "ascii", "ascii_rounded", "modern", "modern_rounded" or "markdown") and controlls the overall style of the table. See [tabled::settings::style](https://docs.rs/tabled/latest/tabled/settings/style/struct.Style.html) for more information.
