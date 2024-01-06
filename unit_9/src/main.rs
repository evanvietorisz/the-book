use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read}; //imports io and Read trait

//function that reads a username from a file and returns it
//if it works. else, returns the kind of error that comes up
fn read_usernmame_from_file() -> Result<String, io::Error> {
    //generic error is io::Error

    let open_file_result = File::open("hello.txt");
    let mut file = match open_file_result {
        Ok(file) => file, //evaluate to file itself
        Err(e) => return Err(e), //return the Result object, not the error itself
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) =>  Ok(username), //return the variant, not username itself
        Err(e) => Err(e), //return Result object in Err variant, not error e itself
    } //no returns needed here because this match is what the function returns
}

pub struct Guess {
    value: i32, //private field
}

impl Guess { 
    //need pub to make methods public

    //this is not an instance method bc it doesn't take in 
    //a reference to an object instance
    //so it is called using ::
    //it is a static method
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess takes a number between 1 and 100!");
        }
        //if we succeed, return a Guess instance with value:value
        Guess {
            value: value,
        }
    }
    //need a getter method bc the fields of a struct are private

    //this is an instance method, so you access it using Guess.value()
    pub fn value(&self) -> i32 {
        self.value
    }
}


//we can use the ? operator to make this more succinct
//? means if Ok unwrap the thing inside it, if Err return the
//Err from the whole functoion
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; //if doesnt work, function will return Err(e)
    let mut username = String::new();

    file.read_to_string(&mut username)?; //returns Err(e) if doesnt work
    Ok(username) //we want to return the output wrapped in an Ok()
}
//note: the ? operator passes errors through the from function,
//which converts errors of any type to the type specified in the function
//declaration

//we can chain expressions with the ? for even more concision
fn read_username_from_file_very_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
//there is a function in std::fs (filesystem) that
//implements all of this in one fell swoop


fn main() {
    /*

    Errors

    rust does not have exceptions (class objects that also distupt 
    flow of program). it has the panic! macro that stops execution 
    and is to be used for errors that you don't want to recover from
    and the Result<T, E> type for errors you DO want to be able to 
    recover from. 

    ### Panic! ###

    you can cause a panic by doing something illegal like a seg fault 
    or by calling panic!

    panics print a message, unwind (?) clean up the stack, and quit.
    you can use an environment var to keep track of call stack when
    the panic occurs

    unwinding means going back through the call stack and freeing the memory
    this takes time. if you were in a production context and didn't want to
    do this, you'd want to set panic to abort mode, which means it doesnt
    unwind. it just quits immediately

    you do this by putting 
    [profile.release]
    panic = 'abort' 

    in the Cargo.toml file

    */


    //let's call panic 
    //panic!("crash and burn")

    //backtraces. helps you trace the source of the problem in your code
    //if the panic! was called in code someone else wrote

    //let's get a backtrace. you do it by setting RUST_BACKTRACE=1 cargo run
    //ie you set some env variable before the cargo run command
    //to get full debug info you need to be running cargo build or cargo run
    //without the --release flag, a thing that modifies the compilation 
    //for production code

    //note: in rust indexing past array isn't undefined behavior; its a fatal error
    let v: Vec<i32> = vec![1,2,3];
    //v[99];

    /*

    ### the Result<T, E> type ###

    used for errors you want to be able to recover from 

    it is an enum type similar to option implemented as 

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    it is the output of many functions that attempt to do something that
    can fail. if the operation is successful, it is the Ok variant
    with something of type T. if it fails, it is the Err variant with 
    an error of type E

    */

    //let's use the result object

    // let greeting_file_result = File::open("hello.txt");
    // //handle both possibilities
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file, 
    //     Err(error) => panic!("Problem opening file: {:?}", error),
    //     //note we are using the debug rendering {:?} here
    // };

    //Result is in the prelude; no need to bring it in 


    //doing different things based on the error
    //if we get a NotFound error trying to open a file, try to create the file
    //and return success or error based on that. if some other error occurs,
    //panic and print that

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file, 
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("problem creating file: {:?}", e),
            },
            //catch-all
            other_error => panic!("problem opening file: {:?}", other_error),
        }
    };

    //io:Error is a struct, which has a kind method, which returns 
    //ErrorKind enum. Note: you access variants of an enum with ::

    //note: you can mitigate the nested match statements using closures
    //TODO come back to this example

    //the Result type has many methods implemented on it that 
    //mak it more succinct to use

    //.unwrap() either returns the file if it is Ok or panics if Err
    let greeting_file = File::open("hello.txt").unwrap();

    //.expect() behaves the same way but lets us choose the panic message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt needs to be in the directory");
    //expect is more production/professional

    // ### propagating errors ###
    //sometimes you want you code to return an error so that code
    //that calls it can decide what to do

    let username: Result<String, io::Error> = read_usernmame_from_file();

    //see above

    //you can only use the ? operator inside functions whose 
    //return type is compatible with returning an Err(e), 
    //aka Result and Option
   
    //you can use the ? on something that returns an Option
    //the behavior is the same; it early returns None if 
    //option is None, else unpacks the value in Some(value)

    //note: you are allowed to modify the main() function 
    //to return a Result<(), Box<dyn Error>> 
   // Result<(),E> note: () is the eqiuvalent of void in rust

    //the Box thing is a trait object. means "any kind of error"

    //main can return types that implement the std::process:termination
    //trait that obeys certain properties. you can define your own if you 
    //want to


    /*
    ### choosing when to use panic or Result<> ###

    Result<> is more professional bc it allows calling code
    to decide whether something going wrong should terminate the process
    
    but it's more verbose, so using panic in examples or test/prototype
    code is easier

    unwrap and expect are quick and dirty ways of adding in 
    panics upon unsiccessful operations, and the idea is that when you 
    are done prototyping, you go in and add more robust error handling

    one key reason is that when you write unit tests in rust, the fail
    if panic is called. so if you want a certain thing going wrong to 
    make a test fail, you need to build in panics. again, this is why
    unwrap and expect are common choices for test code

    it is also okay to use unwrap and expect in a situation
    where you know your code can't fail but the compiler doesnt

    example:

    let IPAddress: String = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid")

    this value is a valid IP address and should never cause an error,
    but the parse() method still returns a Result object. 
    the compiler makes you handle this as a possible source of error
    even though you know it can't be
    so it is considered okay to use expect and just annotate the
    reason you believe it is acceptable not to do further error handling

    you also want to use panic when an error is something that could 
    only come from a bug and is not something you can expect. for example
    someone entering an invalid http address is expected and should result
    in an error code that doesn't crash the program, but someone indexing
    past the end of a data structure should because thats not a valid use
    and the programmer needs to fix it


    You can also use custom types to do error checking 
    imagine an example where you write code that must take in a
    number, and that number needs to be between 1 and 100

    even the u8 type cant ensure this is the case, so what you'd do 
    is create a custom Guess type that in its constructor
    checks if the number you try to initialize it with is valid

    this allows you to write code that takes in a guess where you 
    can be sure that the number passed in is between 1 and 100, saving
    you the trouble of taking in a regular int and having to do 
    error checking all over the place

    */

    let my_guess = Guess::new(3); //nice
    let val = my_guess.value();
    println!("{val}");

    //let my_guess = Guess::new(-4); causes a panic!

}
