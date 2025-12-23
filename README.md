# TODO

_todo_ is a simple todo list cli written in rust.

## Installations

### From source code
Prerequisites: `rustrc`, `cargo` and (optionally) `just` installed.

After having cloned the repo you can simply run
```console
just install
```
This will build the release version from the source code using `cargo build --release`, copy the binary into `~/.cargo/bin` and install the auto-completions.

## First steps
When running _todo_ for the first time, it is recommended to run `todo init`.
It will create the folder `~/.todo/` and creates two files

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
  open         Open a task
  delete       Delete a task
  delete-all   Delete all tasks in the current todo list
  reword       Reword a task
  update       Update the fields of an item
  clear        
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
  -a, --all          Show all tasks
      --done         Show all completed tasks
      --open         Show all open tasks
  -s, --sort <SORT>  Sort tasks
      --collection   Show collection
      --tags         Display available tags
  -h, --help         Print help

```
Since it is used so often, it is the default command that is the following two are equivalent
```console
$ todo 
$ todo list
```

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
Finally, we want to point out the `show` command. It is similar to `git`, the `show` command displays the todo task with more meta data. In particular it shows the body of the task. For example
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
