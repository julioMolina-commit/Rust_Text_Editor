// -- Current part of the tutorial --
// Chapter 3: Assignment 1: Tildes


// The Editor is now mutable:
// Immutable means that it can be read but not modified
// Mutable means it can be read AND modified

// use std::io::{self, Read}; // Uses the 'input and output' module from 'rust standard library', 'self' brings in 'io' itself as  a library
// 'Read' brings in the trait that provides '.bytes()' to read the input
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers}; // Import from the library 'crossterm-event' read, key and keycode
use crossterm::terminal::{disable_raw_mode, enable_raw_mode}; // Uses the Rust library for 'cross-platform terminal manipulation' taking the disabling and enabling of the raw mode of the terminal
// Raw mode being a setting where the input is processed character by character

// ^^^^ The imports ----------------------------------------------
// vvvv The code -------------------------------------------------

pub struct Editor { // Defines a 'public' 'struct' which is empty
// The struct is like a touple that holds data
    should_quit: bool,
}

impl Editor { // Implements methods for Editor (Like class methods)
    pub fn default() -> Self { // The default function called
        Editor{should_quit: false} // Creates and returns an empty editor struct
    }

    pub fn run(&mut self) { // This is the main method that runs in the instance and refers to itself
        if let Err(err) = self.repl(){ // Calls repl() and checks if it returns an error
            panic!("{err:#?}"); // If there is an error it gives a crash with an error
        }
        print!("Goodbye. \r\n"); // When the loop ends says goodbye
    }

    fn repl(&mut self) -> Result<(), std::io::Error>{ // Returns Ok(()) in a success or Err(io::Error) in a failure
        enable_raw_mode()?; // In case of an error return to regular mode in the terminal

        loop {
            if let Key(KeyEvent{
                code, modifiers, kind, state
            }) = read()?
            {
                // Takes the information from the key stroke and turns it into four sets of data, the key code, the modifiers applied, the type of key and the state
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                // If the code introduced is equal to 'q' under the modifier of a control key the program is set to end
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL =>{
                        self.should_quit = true;
                    }
                    _=> (),
                }
            }

            if self.should_quit{
                break;
            }
        }

        disable_raw_mode().unwrap(); // Restores the usual terminal
        Ok(()) // Returns success with an empty tuple
    }
}