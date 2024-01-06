// Unit 2: Programming a Guessing Game

/* 

Let's implement a game where the computer decides a random number, then
the user tries to guess it. The computer tells them if their guess is too
low or too high. If they guess the correct number, the program congratulates 
them and the program exits.

*/

//to read input, we need the std::io package

use std::io; //:: is the path operator, not same as calling an object method
use rand::Rng; //for random number generation
          //since this is not in std, we need to go to the .toml file
          //for this project and add the dependency rand="0.8.5" to Cargo.toml
          //note: this "" syntax says: "0.8.5 <= version < 0.9.0 "
          //means "the newest version of the package that will compile the program"

          //however, the Cargo.lock file contains the exact versions you had at the first build,
          //and specifies what is actually used when you build the project
          //which remain the same until you change them using "cargo update",
          //which accommodates whatever is currently listed in your .toml file 
          
          //note: Rng is a TRAIT of rand
use std::cmp::Ordering; //Enum type, contains the outcomes of a comparison

fn main() {
    println!("Guess a number between 1 and 100!");
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); //special range syntax, inclusive of upper and lower bounds
    
    loop {
        println!("Please input your guess: ");

        //process a guess
        let mut guess = String::new(); //allocate a new String on the heap
        io::stdin() //call the stdin function from io module, which returns a Stdin type, which has methods below
            .read_line(&mut guess) //takes input from user and puts it into guess, following a mutable pointer to it. read_line returns a Result, a type of enum
            .expect("Failed to read line"); //checks whether read_line returned an error.
                //note: read_line *appends* to whatever pointer you give it, not replaces
        
        //convert to integer, shadowing 
        //easy option would be guess.trim().parse() .expect("Please type a number!");
        //idea: make it so that if you input an invalid value, 
        //it just prompts you again. 
        
        //parse() returns a Result object, which is an Enum with variants Ok and Err
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue //_ is a catchall value. we want to match all errors
        };
        
        println!("You guessed {guess}");

        //compare to secret number using match statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}