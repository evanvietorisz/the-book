use std::env;//style: bring in the parent module of the desired function
use std::process;

use minigrep::Config; // minigrep is the current crate, and this
//automatically goes to the library file, the root of the library
//crate's module tree

fn main() {
    /*

    let's create a lightweight version of grep, a command line tool that
    scans a specified file for a regex and returns all the lines
    in the file where the regex is found. 

    */

    //we want to be able to collect command line arguments
    //for the command line tool, i.e. cargo run -- searchstring filename.txt
    //we use the std::env::args() function

    let args: Vec<String> = env::args().collect();
    //dbg!(&args); //& bc otherwise would take ownership
    //args() returns an iterator and collect() puts all elements 
    //it iterates over into a collection like a Vec
    //also works to turn stuff into another collection

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        //e makes it print to stderr
        process::exit(1); // exit(error code) gives less verbose output than panic!
    }); // this is a closure, an anonymous function. the |err|
    //is an argument to it
    //it unwraps a Result if it is Ok and otherwise does something else
    //where the namespace of the {} block contains the err name
    //which is hardcoded to be the thing inside the Err variant

    //logic of program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }//if let says "if the result is an error, do ..."
}



//let's separate all the logic into a separate function
//this leaves the logic in astandalone piece and makes main
//responsible only for error handling the logic of the program

