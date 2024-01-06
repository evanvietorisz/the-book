use std::fs;
use std::error::Error; //necessary for Box<dyn Error>
use std::env; 

pub struct Config {
    pub query: String, //make this String and not &String so we don't have lifetime issues
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //doesnt take a reference to self so is Config::build()
    //note: we call it build() bc people expect new() functions
    //to never fail

    //good practice to make the build return a Result 
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        //takes in a slice of a sequence containing Strings
        //returns a Result that is either a Config in the Ok case
        //or a &str literal of infinite lifetime
        //we specify the lifetime bc otherwise it would be set to 
        //the lifetime of the input reference

        //make sure there are enough args
        if args.len() < 3 {
            return Err("not enough arguments"); //string literal of infinite lifetime
        }

        //note: as in C, a program by default takes its own name as 
        //its first command line argument, i.e. args[0]
        let query: String = args[1].clone(); //clone so we don't have lifetime issues
        let file_path: String = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok(); //is_ok returns false if var isnt set
        //example use:  IGNORE_CASE=1 cargo run WHO poem.txt 

        Ok(Config { 
            query, 
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read file contents
    //the Box<dyn Error> is a trait object and allows us to return
    //any error we want as long as it implements the Error trait

    let contents: String = fs::read_to_string(config.file_path)?;
    //the ? like unwrap_or_else but inside the else you end up 
    //returning Err(err)

    // .expect("Should have been able to find file");
    // //read_to_string returns a Result<String, io:Error>. expect
    // //unwraps the String contained in the Ok variant
    // //and panics if it is an Err variant

    //println!("Text: \n {}", contents);

    // if config.ignore_case == true {
    //     let result: Vec<&str> = search_case_insensitive(&config.query, &contents);
    // } else {
    //     let result: Vec<&str> = search(&config.query, &contents);
    // } DOESNT WORK bc results goes out of scope

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in &result {
        println!("{}", line);
    }

    Ok(())

}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    //vec!["hello world"]
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    // or
    // contents.
    //      .lines()
    //      .filter(|line| line.contains(query))
    //      .collect()

    results
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    //vec!["hello world"]
    let mut results: Vec<&str> = Vec::new();
    let lowercase_query = query.to_lowercase(); //creates a String, not a slice bc creates new data


    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }

    results
}


//let's implement the search feature of our program 
//by using Test Driven Development. we write a test
//that will fail that specifies the functionality
//we want out of our program. then we amend our program until
//it works

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_output() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}