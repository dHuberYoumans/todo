# TODO

_todo_ is a simple todo list cli written in rust.

## Basic Usage
Its UX is following closely the UX of git, in the sense that using _todo_ follows the following simple pattern
```console
$ todo [cmd] [options] [args]
```

## Prerequisites
_todo_ utilizes `sqlite` to create, read and write to a sql database.
On macOS it can be installed via brew
```console
$ brew install sqlite3
```

## First steps
When running _todo_ for the first time, it is recommended to run `todo init`.
This initializes _todo_: it creates the folder `~/.todo/` and creates two files

1. `~/.todo/.env`  
2. `~/.todo/todo.db`

The dotenv file contains a line `TODO_DB=todo.db` and is used to load the list `~/.todo/todo.db`. 
_todo_ allows you to create and load several lists via `todo new-list [name]` and `todo load [name]`, see below.
The loading is done by setting the dotenv variable `TODO_DB=name.db`. 


## todo help
```console
$ todo help

Todo - A simple todo cli so you don't have to leave the terminal

 init                      Initialize the cli
 new-list [name]           Add a new todo list 'name'
 delete-list [name]        Delete the todo list 'name'
 load [name]               Load the todo list 'name'
 add [task]                Add a new task with the given description
 list                      Display all tasks in your current todo list
 open [id]                 Mark the task with the given ID as not done / open
 close [id]                Mark the task with the given ID as done / completed
 delete [id]               Remove the task with the specified ID
 delete-all                Remove all tasks from the todo list
 reword [id] [new task]    Replace the description of the task with the given ID
```

## Shell auto-completion
### Zsh

```bash
todo auto-completions zsh > ~/.zsh/completions/_todo
```

### Bash
```bash
todo auto-completions bash > ~/.local/share/bash-completion/completions/todo
```

### Fish
```bash
todo auto-completions fish > ~/.config/fish/completions/todo.fish
```

Restart your terminal to activate completions.


