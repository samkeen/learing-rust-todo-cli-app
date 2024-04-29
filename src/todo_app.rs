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
    pub(crate) fn complete(&mut self, id: usize) {
        if let Some(item) = self.items.get_mut(id) {
            item.completed = true;
        }
    }

    /// Displays all the todo items in the list.
    ///
    /// The items are displayed with their `id`, completion status, and text.
    /// Completed items are prefixed with `[x]`, while incomplete items are prefixed with `[ ]`.
    ///
    /// The items are written to the provided `writer`, which can be any type that implements the `Write` trait.
    pub(crate) fn list<W: std::io::Write>(&self, writer: &mut W) {
        for item in &self.items {
            let status = if item.completed { "[x]" } else { "[ ]" };
            writeln!(writer, "{} {}: {}", status, item.id, item.text).unwrap();
        }
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
        app.complete(0);
        assert_eq!(app.items[0].completed, true);
        assert_eq!(app.items[1].completed, false);
    }

    #[test]
    fn test_view_todos() {
        let mut app = TodoApp::new();
        app.add(String::from("Buy groceries"));
        app.add(String::from("Do laundry"));
        app.complete(1);

        let mut output = Vec::new();
        {
            let mut writer = std::io::BufWriter::new(&mut output);
            app.list(&mut writer);
        }
        let output_string = String::from_utf8(output).unwrap();
        assert!(output_string.contains("[ ] 0: Buy groceries"));
        assert!(output_string.contains("[x] 1: Do laundry"));
    }
}