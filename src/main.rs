// This is the main file for the text editor using this tutorial (https://philippflenker.com/hecto-chapter-2/)
// ------------- ¡¡¡I SHOULD DOCUMENT EVERYTHING!!! --------

// Functions in Rust | In Rust all the executable code has to be in functions
// The main function vvv is special because it's the starting point of the program (Think of 'Void Start' in Unity)

// When you return from 'main' the program passes the control back to the OS

// Rust is a compiled language so we need to run the program through a compiler to make it executable, we then run it like any other program

// -------- To run the program  --------
// To run the program use 'rustc name.rs' in the terminal
// Rust also comes with 'cargo', it manages dependencies, compiles the code...

// 'cargo --version' shows the version of cargo

// --------- To make the program -------------
// In 'cargo' use 'cargo init name --vcs none'
// '--vcs none' makes it so that you don't initialize git support

// ---------- TO REALLY RUN THE PROGRAM ----------
// After making the folder with 'cargo', use 'cargo build' in the terminal while inside the 'name' folder
// Use then 'cargo run'

// ----------- THE ACTUAL FKNG PROGRAM HERE ---------------

mod editor; // Declares the module 'editor' so Rust knows to look for it
use editor::Editor; // Imports 'editor', 'use' also brings in the names so we don't have to use (editor::Editor) every time we refer to one

fn main() { // This is the main function that is executed as the entry point of the program
    let mut editor = Editor::default(); // Sets the default method of Editor as a variable (Creates an Editor instance with default values (Think of it as an object???))   
    editor.run(); // Runs the main loop of the editor file
}

// When run() is done the program exits