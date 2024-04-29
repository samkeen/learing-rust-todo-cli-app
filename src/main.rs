// bring the contents of todo_app.rs into scope 
mod todo_app;
use todo_app::TodoApp;
///
/// This is the skeleton of  the CLI application which will utilize TodoApp
/// 
fn main() {
    // Create a new instance of TodoApp
    let mut app = TodoApp::new();

    // Display the welcome message and usage instructions
    println!("Welcome to your Todo app");
    // Print out the usage
    usage();

    // Start the main loop. see: https://doc.rust-lang.org/beta/std/keyword.loop.html
    loop {
        // [@@TASK@@] on each loop display the list of todo items using `app.list()` 
        //            Hint: You'll need to loop over the Vec that is returned.
        
        
        print!("> ");
        // [@@TASK@@] Use the io standard lib to prompt for and read user input. Remember it is 
        //            best to trim that input of whitespace.  
        
        
        // [@@TASK@@] Start processing the user's command
        if input.starts_with("add") {
            // Extract the text of the input after the 'command'
            // Add the new todo item to the app
            // Print a success message
        //    
        // [@@TASK@@] continue to process known commands with `else if` for each available command
        //
        } else {
            // [@@TASK@@] Print an error message for invalid commands
        }
        // Ensure the prompt is on a new line in the terminal
        println!();
    }
}

fn usage() {
    // [@@TASK@@] print out the list of available commands including short descriptions
}