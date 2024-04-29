/// Represents a single todo item.
pub(crate) struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

/// Represents the todo application.
pub(crate) struct TodoApp {
    items: Vec<TodoItem>,
}

impl TodoApp {
    /// Creates a new instance of `TodoApp` with an empty list of items.
    pub fn new() -> Self {
        TodoApp { items: Vec::new() }
    }

    /// Adds a new todo item with the given text to the list.
    ///
    /// The `id` of the new item is automatically assigned based on the current number of items.
    pub(crate) fn add(&mut self, text: String) {
        let id = self.items.len();
        let item = TodoItem {
            id,
            text,
            completed: false,
        };
        self.items.push(item);
    }

    /// Marks the todo item with the specified `id` as completed.
    ///
    /// If the `id` is valid and corresponds to an existing todo item,
    /// the `completed` flag of that item is set to `true`.
    /// 
    /// Returns bool signalling if todo item was found
    pub(crate) fn complete(&mut self, id: usize) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.completed = true;
            return true
        }
        false
    }

    /// Returns a vector of formatted strings for all todos.
    ///
    /// This function iterates over each todo in the todo list, and converts it into a formatted string.
    /// The returned vector maintains the original order of todos.
    ///
    /// Each string is constructed as:
    /// * "[x]" + " :" + <todo id> + " " + <todo text> for completed todos, or
    /// * "[ ]" + " :" + <todo id> + " " + <todo text> for not completed todos.
    ///
    /// # Returns:
    ///
    /// * `Vec<String>` - A vector of formatted strings representing all todos.
    pub(crate) fn list(&self) -> Vec<String> {
        self.items.iter().map(|todo| {
            let status = if todo.completed { "[x]" } else { "[ ]" };
            format!("{} :{} {}", status, todo.id, todo.text)
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut app = TodoApp::new();
        app.add(String::from("Buy groceries"));
        assert_eq!(app.items.len(), 1);
        assert_eq!(app.items[0].id, 0);
        assert_eq!(app.items[0].text, "Buy groceries");
        assert_eq!(app.items[0].completed, false);
    }

    #[test]
    fn test_complete_todo() {
        let mut app = TodoApp::new();
        app.add(String::from("Buy groceries"));
        app.add(String::from("Do laundry"));
        let todo_found = app.complete(0);
        assert!(todo_found);
        assert_eq!(app.items[0].completed, true);
        assert_eq!(app.items[1].completed, false);
    }

    #[test]
    fn test_complete_todo_not_found() {
        let mut app = TodoApp::new();
        app.add(String::from("Buy groceries"));
        app.add(String::from("Do laundry"));
        let todo_found = app.complete(99);
        assert_eq!(todo_found, false);
    }

    #[test]
    fn test_view_todos() {
        let mut app = TodoApp::new();
        app.add(String::from("Buy groceries"));
        app.add(String::from("Do laundry"));
        app.complete(1);

        let output_vec = app.list();
        assert!(output_vec.contains(&"[ ] :0 Buy groceries".to_string()));
        assert!(output_vec.contains(&"[x] :1 Do laundry".to_string()));
    }
}