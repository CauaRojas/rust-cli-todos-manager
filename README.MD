Rust Todos
==========

A command-line tool to manage your todos.

Usage
-----

```shell
rust-todos [COMMAND] [VALUE]
```

Commands
--------

* **Add** `"TODO TEXT"`: Adds a new todo to the list.
* **Remove** `NUMBER`: Removes a numbered todo from the list (use the `list` command to get the todo number).
* **List**: Lists all todos in order.

Dependencies
------------

* serde: 1.0
* serde\_json: 1.0

Installation
------------

To use this tool, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

1.Clone this repository:

```shell
git clone https://github.com/your/repo.git
```

2.Change into the project directory:

```shell
cd rust-todos
```

3.Build the project:

```shell
cargo build --release
```

4.Run the tool:

```shell
rust-todos [COMMAND] [VALUE]
```

Examples
--------

*Add a new todo:

```shell
rust-todos add "Buy groceries"
```

*Remove a todo:

```shell
rust-todos remove 1
```

*List all todos:

```shell
rust-todos list
```

*Show help:

```shell
rust-todos help
```

License
-------

This project is licensed under the [MIT License](LICENSE).
