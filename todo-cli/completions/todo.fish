# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_todo_global_optspecs
	string join \n h/help V/version
end

function __fish_todo_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_todo_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_todo_using_subcommand
	set -l cmd (__fish_todo_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c todo -n "__fish_todo_needs_command" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_needs_command" -s V -l version -d 'Print version'
complete -c todo -n "__fish_todo_needs_command" -f -a "init" -d 'Initialize the cli in CWD'
complete -c todo -n "__fish_todo_needs_command" -f -a "config" -d 'Open config'
complete -c todo -n "__fish_todo_needs_command" -f -a "new-list" -d 'Create a new todo list'
complete -c todo -n "__fish_todo_needs_command" -f -a "delete-list" -d 'Delete a todo list'
complete -c todo -n "__fish_todo_needs_command" -f -a "load" -d 'Load a todo list'
complete -c todo -n "__fish_todo_needs_command" -f -a "whoami" -d 'Print the name of the todo list in use to stdout'
complete -c todo -n "__fish_todo_needs_command" -f -a "add" -d 'Add a task'
complete -c todo -n "__fish_todo_needs_command" -f -a "list" -d 'Print the current todo list'
complete -c todo -n "__fish_todo_needs_command" -f -a "show" -d 'Show metadata of a task'
complete -c todo -n "__fish_todo_needs_command" -f -a "close" -d 'Mark a task as completed'
complete -c todo -n "__fish_todo_needs_command" -f -a "open" -d 'Open a task'
complete -c todo -n "__fish_todo_needs_command" -f -a "delete" -d 'Delete a task'
complete -c todo -n "__fish_todo_needs_command" -f -a "delete-all" -d 'Delete all tasks in the current todo list'
complete -c todo -n "__fish_todo_needs_command" -f -a "reword" -d 'Reword a task'
complete -c todo -n "__fish_todo_needs_command" -f -a "update" -d 'Update the fields of an item'
complete -c todo -n "__fish_todo_needs_command" -f -a "clear"
complete -c todo -n "__fish_todo_needs_command" -f -a "show-paths" -d 'Show user paths'
complete -c todo -n "__fish_todo_needs_command" -f -a "clean-data" -d 'Clean data'
complete -c todo -n "__fish_todo_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c todo -n "__fish_todo_using_subcommand init" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand config" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand new-list" -s c -l checkout -d 'Directly load new list'
complete -c todo -n "__fish_todo_using_subcommand new-list" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand delete-list" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand load" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand whoami" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand add" -s m -l task -d 'Task description' -r
complete -c todo -n "__fish_todo_using_subcommand add" -s p -l prio -d 'Priority' -r -f -a "p1\t''
p2\t''
p3\t''
empty\t''"
complete -c todo -n "__fish_todo_using_subcommand add" -s d -l due -d 'Due date' -r
complete -c todo -n "__fish_todo_using_subcommand add" -s t -l tag -d 'Tag' -r
complete -c todo -n "__fish_todo_using_subcommand add" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand list" -s s -l sort -d 'Sort tasks' -r
complete -c todo -n "__fish_todo_using_subcommand list" -s a -l all -d 'Show all tasks'
complete -c todo -n "__fish_todo_using_subcommand list" -l done -d 'Show all completed tasks'
complete -c todo -n "__fish_todo_using_subcommand list" -l open -d 'Show all open tasks'
complete -c todo -n "__fish_todo_using_subcommand list" -l collection -d 'Show collection'
complete -c todo -n "__fish_todo_using_subcommand list" -l tags -d 'Display available tags'
complete -c todo -n "__fish_todo_using_subcommand list" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand show" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand close" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand open" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand delete" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand delete-all" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand reword" -s m -l task -d 'Task description' -r
complete -c todo -n "__fish_todo_using_subcommand reword" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand update" -s d -l due -d 'Update the due date' -r
complete -c todo -n "__fish_todo_using_subcommand update" -s p -l prio -d 'Update the priority' -r -f -a "p1\t''
p2\t''
p3\t''
empty\t''"
complete -c todo -n "__fish_todo_using_subcommand update" -s s -l status -d 'Update the status' -r -f -a "closed\t''
open\t''"
complete -c todo -n "__fish_todo_using_subcommand update" -s t -l tag -d 'Update the tag' -r
complete -c todo -n "__fish_todo_using_subcommand update" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand clear" -l due -d 'Clear the due column'
complete -c todo -n "__fish_todo_using_subcommand clear" -l prio -d 'Clear the prio column'
complete -c todo -n "__fish_todo_using_subcommand clear" -l tag -d 'Clear the tag column'
complete -c todo -n "__fish_todo_using_subcommand clear" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand show-paths" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand clean-data" -s h -l help -d 'Print help'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "init" -d 'Initialize the cli in CWD'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "config" -d 'Open config'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "new-list" -d 'Create a new todo list'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "delete-list" -d 'Delete a todo list'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "load" -d 'Load a todo list'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "whoami" -d 'Print the name of the todo list in use to stdout'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "add" -d 'Add a task'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "list" -d 'Print the current todo list'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "show" -d 'Show metadata of a task'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "close" -d 'Mark a task as completed'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "open" -d 'Open a task'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "delete" -d 'Delete a task'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "delete-all" -d 'Delete all tasks in the current todo list'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "reword" -d 'Reword a task'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "update" -d 'Update the fields of an item'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "clear"
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "show-paths" -d 'Show user paths'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "clean-data" -d 'Clean data'
complete -c todo -n "__fish_todo_using_subcommand help; and not __fish_seen_subcommand_from init config new-list delete-list load whoami add list show close open delete delete-all reword update clear show-paths clean-data help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
