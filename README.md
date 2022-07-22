# todo-rs

A simple Rust "todo" CLI app that uses the [clap_v3](https://docs.rs/clap-v3/3.0.0-beta.1/clap_v3/) crate and the [diesel](https://diesel.rs/) ORM.

## Installation

To install, clone the repo, `cd` into todo-rs and run:

```{bash}
make install
```

Or alternatively, run:

```{bash}
cargo install --path .
```

Additionally, the app uses a postgres docker container for storage. To spin up the container, run:

```{bash}
docker-compose up -d
```

> *Note*: You will need to have [docker compose](https://docs.docker.com/compose/install/) installed to execute the previous command.

## Usage

Usage instructions can be accessed by running:

```{bash}
todo-rs --help
```

```{bash}
USAGE:
    todo-rs <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add         Add a new todo
    complete    Complete an existing todo
    delete      Delete an existing todo
    help        Print this message or the help of the given subcommand(s)
    list        List existing todos
    update      Update an existing todo
```

### Adding a new todo

```{bash}
todo-rs add --name "feed the cat"
```

### Listing todos

```{bash}
todo-rs list
```

List completed todos:

```{bash}
todo-rs list --completed
```

### Update a todo

```{bash}
todo-rs update --id 1 --name "feed the cat when I get home"
```

> *Note*: The `id` of the todo can be found by running `todo-rs list`

### Complete a todo

```{bash}
todo-rs complete --id 1
```

`todo-rs list` would no longer show any pending todos but running `todo-rs list --completed` will show the completed todos.

### Delete a todo

```{bash}
todo-rs delete --id 1
```
