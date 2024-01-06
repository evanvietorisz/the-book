fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests { //makes tests a different module
    use super::*;  // brings in all definitions from parent file/library

    #[test] //declares this is a real test and is not an aux function
            //to facilitate testing or create correct state
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        //we also have assert_ne! for inequality
        //for both of these order of params doesnt matter
        //when tests with these fail, arguments are printed
        //using the Debug trait. So anything you pass in must
        //implement the PartialEq trait for the comparison and the 
        //Debug trait for printing
    }

    //tests fail when they panic
    //each test is run in a different thread, and is deemed to have
    //failed when the main thread notices it has died
    #[test]
    fn it_doesnt_work() {
        panic!("make this test fail");
    }

    #[test]
    fn using_assert() {
        //the assert! macro takes in an expression that evals to a bool
        //and panics if it doesnt eval to true
        assert!(
            2==2,
            "You can insert a custom error message with a placeholder {}",
            "a val to go in the placeholder",
        );
        //you can do the same with assert_eq! and assert_ne!
    }
}

//you can call cargo test inside this project to run these tests

//let's write tests on a more real use case

#[derive(Debug)] //gives it the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn same_size() {
        let rect1 = Rectangle{
            width: 5,
            height: 9,
        };
        let rect2 = Rectangle{
            width: 5,
            height: 9,
        };
        assert!(rect1.can_hold(&rect2));
        assert!(rect2.can_hold(&rect1));
    }
}

//we can create tests that only pass if a function panics for a particular reason
#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    //use the should_panic attribute 
    #[should_panic(expected="dog")] //a substring of the expected error message 
                                    //to disambiguate it with other reasons
                                    //to panic 
    fn this_function_should_panic() {
        panic!("this particular error message contains the word dog");
    }
    //you can also write tests that return a result
    fn returns_a_result() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("this function returned an error"))
        }
    }//the idea is that this is convenient for functions that 
    //use the ? operator and Result objects to begin with
}





fn main() {
    /*

    ### Writing automated tests ###

    a test is a function that is annotated with the test attribute.
    attributes are metadata tags associated with pieces of rust code.

    you add #[test] before the fn declaration

    you run test with the cargo test command


    ### controlling how tests are run ###
    when you do cargo test, it runs all tests in parallel
    and capture all the output from the tests

    you can pass in command line arguments that relate to cargo test
    or the resulting binary. you do
    cargo test <things that apply to cargo test> -- <things that apply to binary>

    if you want all the tests to run in sequence and not 
    use multithreading, use cargo test -- --test-threads=1
    good thing to do if tests share state

    normally tests capture all output to stdout 
    except the message that gets printed at a panic!
    to undo this, use cargo test -- --show-output

    we can run specific tests by doing 
    cargo test <test_name>

    or cargo test <fragement of test name> 
    all tests for which the fragment is a substring of the test name 
    will be run

    we can also leave out some tests by default by adding the 
    #[ignore] attribute above them
    these tests will not be run unless specifically requested by name

    we can specifically run all ignored tests with 
    cargo test -- --ignored

    we can run all tests, both ignored and unignored, with 
    cargo test -- --include-ignored

    

    ### organizing tests ###

    Unit tests test a specific piece of code in a self-contained way
    Integration tests test your code through the public API and use it 
    however another third party piece of software would


    Unit tests conventially go in the same files as the functions they 
    test. you preface the mod test {} that contains them with
    the #[cfg(test)] attribute

    they are not compiled to binary when you do cargo run; only when 
    you do cargo test

    the #[cfg(test)] tells rust that the following module should
    only be included in compiled binary when the 'test' *configuration
    option* is selected

    note: tests can access both public and private methods/functions
    in the parent script


    Integration Tests

    the idea is to test how pieces of your public API work together

    you create a tests directory alongside your src directory
    in it, you put your integration_tests.rs file 
    inside that, you import all the modules you need and just 
    write tests using the [#(test)] syntax as you would in a unit test

    you don't need to use the #[cfg(test)] attribute bc the fact that
    the functions are in the tests directory tells rust to compile 
    them only when you are in test mode 


    when you call cargo test, tests get run in the order:
    unit tests, integration tests, doc tests
    (the latter tests that the code samples in your documentation work)

    if you have any issues in unit tests, you won't progress to integration
    tests, and so on

    you can call specific integration test files by name using 
    cargo test --test integration_test <- the name of a file in /tests

    
    sometimes you want to create code that will be shared among differnet 
    intgration test files that is not itself an integration test file

    what you would do is create a directory inside the tests directory 
    and then put a file in there that would have code you wanted to share
    then you can import it into any of your integration test files without
    that file getting compiled at testing itself

    note: you cannot import functionality in the main.rs function 
    into an integration test file or otherwise. if a file is meant to be 
    converted to binary, it cant be used as a library

    you need another lib.rs file to contain the functionality you want to 
    import into integration test files or otherwise

    













    






    */
}
