# Learning Rust: A Todo CLI application

This repo accompanies the blog post [Building your first Rust app: an applied learning approach](https://samkeen.dev/building-your-first-rust-app-an-applied-learning-approach).

The goal is to provide a practical, hands-on introduction to Rust by building a simple Todo CLI application.

Rather than taking you step by step through the complete code to build the app, I've provided the requirements for the
application plus a set of unit tests which should pass given a correct implementation.

Here is a sneak peek of what you'll build, see the [blog post](https://samkeen.dev/building-your-first-rust-app-an-applied-learning-approach) for the complete guide.

## CLI Usage

```bash
#
# Starting the app greets the user and displays the help menu
#
$ cargo run

Welcome to your Todo app
Available commands:
  add <todo_text>    - Add a new todo item
  complete <todo_id> - Mark a todo item as completed
  list               - List all todo items
  ?                  - Show available commands
  x                  - Exit the program

>

#
# Adding a todo performs that action then lists all todos
#
> add Finish homework
Todo item added successfully!

[ ] 0: Buy groceries
[ ] 1: Do laundry
[ ] 2: Finish homework

#
# Completing a todo performs that action then lists all todos
#
> complete 0
Todo item marked as completed!

[x] 0: Buy groceries
[ ] 1: Do laundry
[ ] 2: Finish homework

#
# Attempt to complete a nonexistent Todo
#
> complete 99
Todo item with id 99 not found

[x] 0: Buy groceries
[ ] 1: Do laundry
[ ] 2: Finish homework

>

#
# x, exits the application
#
> x
Exiting...
$
```

## Requirements
This is a simple Todo list application accessible through a CLI as seen above.

* Define a `TodoItem` struct with the following fields:

    * `id: <appropriate int type>`: The unique identifier of the todo item, which is the index of the item in the vector.

    * `text: String`: The text description of the todo item.

    * `completed: bool`: A boolean flag indicating whether the item is completed or not.

* Define a `TodoApp` struct that holds a vector of `TodoItem` structs.

    * `items: Vec<TodoItem>`

* Implement the following methods for the `TodoApp` struct. Parameters and return types are described but not given in the function signature. See the Unit tests for more detail of the expected implementations.  
  *Note:* [*These docs*](https://doc.rust-lang.org/rust-by-example/fn/methods.html) *are a good overview of methods used with Structs in Rust. Key is know when to pass a reference to self (*`&self`*) and when that reference would need to be mutable (*`&mut`*).*

    * `fn new()`: Creates a new instance of `TodoApp` with an empty vector of items.

    * `fn add()`: Adds a new todo item with the provided text to the list. The `id` of the new item will be its index in the vector.

    * `fn complete()`: Marks the todo item with the specified `id` as completed by setting its `completed` flag to `true`. Returns `true` if the todo was found, `false` if not found.  
      *Note: Todo items are never removed from the vector but rather marked as completed.*

    * `fn list()`: Returns a list of all the todo items, showing their `id`, completion status, and text. Prefix completed items with `[x]` and incomplete items with `[ ]`.  
      *Note: For simplicity there will be no durable storage; todo items will be stored in a runtime vector (*`Vec`*) and all lost when the program exits.*

* Handle error cases gracefully, such as when the user enters an invalid `id` for completion.

* Implement a CLI interface to interact with the `TodoApp` struct. The CLI will loop until the user chooses the exit command. See the CLI usage example above.

    * The CLI should handle the following commands

    * `add <todo_text>` : This will add a new Todo item

    * `complete <todo_id>` : This will complete the task for the given id. An error should be shown if the Todo id is not found.

    * `list` : This will list all the Todo's with their id and completion status

    * `?` This will list the commands available

    * `x` : This will exit the program