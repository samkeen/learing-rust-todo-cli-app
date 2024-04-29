/// Represents a single todo item.
pub struct TodoItem {}

/// Represents the todo application.
pub struct TodoApp {}

impl TodoApp {}

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