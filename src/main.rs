mod editor; // Declares the module 'editor' so Rust knows to look for it
use editor::Editor; // Imports 'editor', 'use' also brings in the names so we don't have to use (editor::Editor) every time we refer to one

fn main() { // This is the main function that is executed as the entry point of the program
    let mut editor = Editor::default(); // Sets the default method of Editor as a variable (Creates an Editor instance with default values (Think of it as an object???))   
    editor.run(); // Runs the main loop of the editor file
}

// When run() is done the program exits
