## What's Changed


### Bug Fixes

- display help text for clear command


### Documentation

- update README

- update README

- update README

- update README


### Features

- add close-all command

- add upgrade command

- allow multiple due date input formats

- feat!(due date): let config handle due date input and display format


### Refactoring

- use filter option instead of boolean arguments



## What's Changed


### Chores

- Release todo version 0.8.0


### Documentation

- update README


### Features

- feat!(config): let todo.config handle the display of due and tag column



**Full Changelog**: v0.7.0 → v0.8.0## What's Changed


### Chores

- Release todo version 0.7.0

- update justfile

- remove script to bump version


### Documentation

- update README

- update README


### Features

- feat!(due date): add due date format option to config

- display weekdays instead of date within 1 week

- add aliases for priority


### ci

- fix release ci



**Full Changelog**: v0.6.1 → v0.7.0## What's Changed


### Chores

- Release todo version 0.6.1

- add crate metadata required for releases

- integrate git-cliff with cargo-release

- update gitignore

- add install script


### Documentation

- update README


### ci

- add automated release workflow for macOS and Linux



**Full Changelog**: v0.6.0 → v0.6.1## What's Changed


### Bug Fixes

- fixup! refactor(UserPaths): expose user's default config path


### Chores

- update justfile

- fix bump-version script


### Documentation

- update README

- update README

- add changelog


### Features

- add install completions command


### Refactoring

- expose user's default config path



**Full Changelog**: v0.5.1 → v0.6.0## What's Changed


### Chores

- bump version to 0.5.1


### Documentation

- update


### Features

- move to runtime completion build



**Full Changelog**: v0.5.0 → v0.5.1## What's Changed


### Bug Fixes

- fix sorting when listing by tag or due date


### Chores

- bump version to 0.5.0

- bump version to 0.4.1


### Features

- enable autocompletion

- support git-like split of title and message


### style

- change title color



**Full Changelog**: v0.4.0 → v0.5.0## What's Changed


### Bug Fixes

- print correct error message if id not found

- accept P1 for prio flag

- use same table style in list_by_due_date and list_by_tag


### Chores

- bump version to 0.4.0

- remove verbose flag

- print status in white if closed

- rename who-is-this to whoami


### Features

- add show command

- add clear command

- print table after update commands

- allow batch updates

- set table style from config

- add colors


### Refactoring

- move epoch function from schema to Datetime

- split command and repo

- use uuid instead of hashing for ids

- split plumbing and porcelain commands

- let table crate handle display of id

- extract table logic into dedicated table module



**Full Changelog**: v0.3.0 → v0.4.0## What's Changed


### Bug Fixes

- write default config file on first init


### Chores

- bump version to 0.3.0

- include script to bump up the project's version number

- fix sort by tags

- bump version to 0.2.8


### Features

- add an update function

- set up justfile

- add `prefix_id_length` to config

- hash id

- set default sort key in config

- list items corresponding to a tag


### Refactoring

- use actual timestamp for Datetime

- move init under src/commands/plumbing

- move concerns of user paths to UserPaths struct

- implement repository pattern

- improve project's architecture

- refactor query logic

- improve structure of commands

- let Config handle config

- read .env from main

- simplify code


### Tests

- write unit tests for sqlite repositories

- add tests for listing todos by due date and tags



**Full Changelog**: v0.2.7 → v0.3.0## What's Changed


### Bug Fixes

- tag display and integration with sql


### Chores

- bump version to 0.2.7


### Features

- list @due_date



**Full Changelog**: v0.2.6 → v0.2.7## What's Changed


### Chores

- bump version to 0.2.6


### Features

- list all tags



**Full Changelog**: v0.2.5 → v0.2.6## What's Changed


### Chores

- bump version to 0.2.5


### Features

- sort by tag



**Full Changelog**: v0.2.4 → v0.2.5## What's Changed


### Chores

- bump version to 0.2.4

- use unicode chars

- add github workflows

- bundle rusqlite

- add logs


### Features

- add tag support


### Refactoring

- apply proper formating


### Tests

- expand test suite



**Full Changelog**: v0.2.3 → v0.2.4## What's Changed


### Chores

- bump version to 0.2.3


### Features

- open config via 'config' command



**Full Changelog**: v0.2.2 → v0.2.3## What's Changed


### Chores

- bump version to 0.2.2


### Features

- use '--collection' flag to print collection



**Full Changelog**: v0.2.1 → v0.2.2## What's Changed


### Bug Fixes

- prevent loading of non-existing list

- don't allow to delete the list if currently in use


### Chores

- bump version to 0.2.1

- update gitignore


### Features

- load last list with '-'


### Tests

- add integration test for `init` command



**Full Changelog**: v0.2.0 → v0.2.1## What's Changed


### Chores

- bump up version to v0.2.0


### Refactoring

- use single database with several table | use config file


### ui

- improve readibility of the help menu



**Full Changelog**: v0.1.5 → v0.2.0## What's Changed


### Features

- allow reword to using the editor


### Refactoring

- use proper types



**Full Changelog**: v0.1.4 → v0.1.5## What's Changed


### Features

- support editor tasks/messages

- use --task, -m flag for writing task/message directly

- add options 'tomorrow' and 'today'



**Full Changelog**: v0.1.3 → v0.1.4## What's Changed


### Features

- allow sort option in list command

- allow prio and due dates

- provide option to laod new list when created

- add new command who-is-this



**Full Changelog**: v0.1.1 → v0.1.3## What's Changed


### Chores

- update cargo + bump up version


### Features

- add flags to list command


### Refactoring

- use clap for cmdline arg parsing

- handle errors


### ui

- wrap long text

- improve logs



**Full Changelog**: v0.1.0 → v0.1.1## What's Changed



<!-- generated by git-cliff -->
