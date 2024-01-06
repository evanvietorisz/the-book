fn main() {

    /*

    Generics, Traits, and Lifetimes
    

    Generics allow you to define functions and classes that take in objects
    of any type so you don't have to duplicate code

    traits allow you to specify what kinds of things your types can do
    so you can build things that only take in types with specific 
    abilities without knowing exactly what they are

    lifetimes allow you to specify how references interact


    */

    // ### Generics ###

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let mut largest: &i32 = &number_list[0]; //reference not strictly necessary
    for number in &number_list {
        if number > largest {
            largest = number; 
        }
    }
    println!("the largest number is {}", largest);

    //let's implement this functionality in a functio

    fn find_largest_element(list: &[i32]) -> &i32 {
        //note: the input type is a 'sequence' input
        //it accepts vecs, stack arrays, and slices of them
        
        let mut largest: &i32 = &list[0]; //need & bc we want a reference to the element 
        for number in list { //dont need & because list is a reference already
            if number > largest {
                largest = number;
            }
        }
        largest
    }
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let largest_element = find_largest_element(&number_list);
    println!("largest element is {}", largest_element);

    //let's make a version of this function that could work on a
    //sequence of chars or some other object

    //"this function is generic over some type T"
    fn find_largest_element_2<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest: &T = &list[0];
        for element in list {
            if element > largest {
                largest = element; 
            }
        }
        largest
    }
    //the std::... is because this function will fail on any type
    //that can't be ordered or compared using >
    //this syntax says "any type T that implements the PartialOrd trait
    //defined in the standard library"

    //we can now use the function both on lists of ints and chars

    let number_vec: Vec<i32> = vec![34, 50, 25, 100, 65];
    let char_array: [char; 5] = ['h','e','l','l','o'];
    println!("{}", find_largest_element_2(&number_vec));
    println!("{}", find_largest_element_2(&char_array));


    //we can also use generic types in struct definitions

    struct Point<T> {
        x: T,
        y: T,
    } //whatever the generic type T is, both x and y have to be it

    //to have multiple generic types, we need to declare a struct
    //with multiple generic parameters

    struct Point2<T, U> {
        x: T,
        y: U,
    } 
    //note: choices for letters are up to you
    //T is conventional, but recall the definition of 
    //HashMap<K, V> where we use letters that are suggestive of
    //the terms key and value

    //you can also use generics in enums
    /*
    
    recall the definiton of the option type
    enum Option<T> {
        Some(T),
        None
    }
    
    or the Result type
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    //you can also use generics in methods

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    //you need the <T> after impl so that you can use it in the 
    //thing that comes after Point

    let p: Point<i32> = Point {
        x: 5,
        y:10,
    };
    println!("{}", p.x);

    //we can also implement methods that only work on specific types
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }//these methods for power and square root only work on floats
    //so this method is only held by Points of type Point<f32>

    //using generic types has no runtime cost because the compiler replaces
    //them with special types according to the specific type they
    //are called with


    // ### Traits ###

    /*

    a trait is a particular sort of functionality a type can have. 
    we can use trait bounds to specify that a generic type can be 
    any that has certain behavior

    a trait is behavior, and behavior is methods

    a trait is a set of methods we can endow multiple types
    with the ability to do

    */

    //let's imagine we have many types corresponding to different
    //kinds of pieces of text. say we want to summarize all of them

    //let's implement the Summary trait and then give it to all of them

    //a trait is a collection of method signatures
    pub trait Summary {
        fn summarize(&self) -> String; //end in semicolon
    }
    //pub makes it so that other things that import this can access it 

    //now each type that wants to have the Summary trait needs to 
    //implement the summarize method

    //now let's implement a trait on two types type

    pub struct NewsArticle {
        pub headline: String, //pub makes it so these fields can be accessed
        pub location: String, //by other code, not just instance methods
        pub author: String,
        pub content: String,
    }

    pub struct Tweet {
        pub username: String, //pub/?
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for NewsArticle { //impl Trait for Type
        fn summarize(&self) -> String {
            //don't need pub bc Summary trait is already public
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    //now these types both implement the Summary trait,
    //but do so in different ways

    //to call a trait method, you need to bring the trait into scope

    //so say the trait and these types were implemented in a 
    //crate (code file) called text_objects

    //then we would import them by doing
    // use text_objects::{Tweet, NewsArticle, Summary};

    // other ppl could also import the Summary trait and implement
    //it on theri own types

    //to implement a trait on a type, either the type or the trait or
    //both need to be defined in the crate in question

    //like we could define our own type and implement the display trait
    //on it in the same file

    //we could also define our own trait and implement it on the Vec
    //type in the same file

    //but we can't  randomly implement a trait that is defined elsewhere
    //on a type that is defined elsewhere

    //we can also make defailt implementations of certain traits

    pub trait Author {
        fn author(&self) -> String {
            String::from("default implementation")
        }
    } 

    //types can implement this default behavior using 

    impl Author for Tweet {} //empty implementation bloc

    //or overwrite it to create their own

    //traits can implement methods that refer to other methods in the trait

    //we can implement a custom method that appeals to default 
    //implementation of another method in the trait

    //if you do this for a type, it also gains access to the default
    //method you referenced in the custom method



    //you can use trait bounds when declaring functions

    //let's build a function that only works on types that 
    //implement the Summary trait

    pub fn notify<T: Summary>(item: &T) {
        println!("breaking news: {}!", item.summarize());
    }

    //the <T: Summary> says "takes in any type that implements the Summary trait"

    //we can express the same using some syntax sugar

    pub fn notify_2(item: &impl Summary) { //takes in a type that implements Summary trait
        println!("breaking news: {}!", item.summarize());
    }
    //note: if we had 2 things, item1 and item2, the above 
    //would specify that they are both the same type &T,
    //while the bottom would allow them to be different types
    //that both implement the Summary trait

    //we can replace 'Summary' in both examples with
    //'Summary + OtherTrait' to specify dependence on multiple traits

    //note: the idea of specifying that a generic type must implement 
    //a particular trait is "specifying trait bounds"

    //we have one more way to specify trait bounds in a 
    //way that doesnt clutter up the first line of the 
    //function declaration 

    pub fn some_function<T, U>(item1: &T, item2: &U) -> i32
    where
        T: Summary, // + whatever else
        U: Summary, // + whatever else
    {
        //function body goes here 
        3 //just return some random value
    }

    //you can also implement functions that return a trait object
    // fn returns_summarizable() -> impl Summary {...}
    //this is often used with closures and iterators 
    //because it allows you to specify that your return value
    //implements a particular trait without specifying 
    //a long iterator type or whatever a closure creates

    //implementing methods conditionally
    //say we want to implement a method on a type that takes in 
    //generic types, but only in cases where those generic types
    //obey certain properties. this is where we use conditional 
    //method implementation
    
    //let's implement a generic pair of objects

    struct Pair<T> {
        x: T,
        y: T,
    }

    //now let's implement a method on it that only works
    //if the T type implements the summarize method

    impl<T: Summary> Pair<T> {
        fn summarize_both_elements(&self) -> String {
            format!("x: {} -- y: {}", self.x.summarize(), self.y.summarize())
        }
    }

    //we can also conditionally implement a trait for any type that 
    //implements another trait

    // impl<T: Summary> Summary2 for T {
    //  more function definitions...
    // }
    //a use case would be to implemnet a to_string() method
    //on all types that implement the display trait 




    // ### Lifetimes ###

    

    /*

    the following code is bad because r points to x even after
    x goes out of scope aka is dropped: 

    fn main(){
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }

    the way the compiler knows this is bad is by using the borrow checker,
    which basically notes that the lifetime of r (from its declaration
    to the end of the program) is bigger than the lifetime of the thing 
    it points to (within the smaller set of brackets)

    consider

    fn main() {
        x = BLAH;
        y = BLAH;

        longest = return_longer(&x, &y);
    }

    where longer compares the inputs and returns the one 
    that is longer

    this poses a problem for the borrow checker because now it can't 
    tell from first glance whether the thing that gets spit out
    into longest is x or y, so it cant make sure any references to them
    don't exist longer than they do. longest becomes one of x or y,
    and could continue to exist after the x and y variables don't anymore,
    but the borrow checker has no way of knowing which one is being passed
    into longer, so it can't do checking on references that point to that
    memory

    the way we solve this is by annotating a generic lifetime in the longer function

    lifetimes dont change how long references exist; they just
    denote relationships among the lifetimes of different references
    so the borrow checker can track them

    */

    // WILL NOT WORK
    // fn return_longer(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    fn return_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let x: &str = "hello";
    let y: &str = ", world!";

    let longest: &str = return_longer(x, y);

    //generic lifetime parameters are generic, so like generic type 
    //parameters, the go in <> in between the function name and its 
    //params specific in (). 

    //you name them using an apostrophe and a very short, usually 
    //lowercase name like 'a.

    //in the function above, we specify that the output of the function
    //lives (at least) as long as both the inputs, so if they both
    //have lifetime 'a, so does the output

    //we write the lifetimes of references after the & in all 
    //type annotations

    //&i32 - a regular reference
    //&mut i32 a regular mutable reference
    //&'a i32 - a reference with lifetime 'a
    //&'a mut i32 - a mutable reference with lifetime 'a 

    //so when we implement the annotation, it tells the compiler
    //that the output lives as long as the smaller of the lifetimes
    //of the two inputs

    //when the function is actually called, 'a will be
    //substituted for the smaller of the lifetimes between x and y

    //annotations go in the function signature, not body

    //let's look at an example of how this would catch you in an error

    // let string1 = String::from("long string is long");
    // let result; //to make it have a longer lifetime
    // {
    //     let string2 = String::from("xyz");
    //     result = return_longer(string1.as_str(), string2.as_str());
    // }
    // println!("the longest string is {}", result);

    //this code doesnt work because the lifetime of result is the 
    //minimum of the two lifetimes of string1 and string2, and 
    //string2 goes out of scope at the end of the curly braces
    //so result is not longer a valid reference after that


    //you only need to annotate references that make sense
    //if you had a function that took in two references
    //and always returned the first one, then you'd only 
    //need to annotate the lifetimes of the first param and the output
    //as equal, since the lifetime of the second param wouldn't have 
    //a relationship to either of them

    //another thing: you can't return a reference to a local var in a 
    //function

    // fn longest(x: &str, y: &str) -> &str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    //this fails bc result is a local variable that gets cleaned up
    //at the end of the function call, so you can't return a reference
    //to it, as that would be a dangling reference.
    //you'd have to return the object itself so that ownership would
    //be transferred to the calling function

    //we can also put lifetime annotations in the parameters of 
    //structs. this is for when structs take in references 

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me. maybe. hey. I just met you...");
    let first_sentence = novel.split('.').next().expect("could not find a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //lifetime elision rules
    //you don't always need to specify lifetimes if they can be inferred
    //every reference as an input to a function has an implicit lifetime
    //annotation under the hood. if there is one input, all outputs
    //get that lifetime
    //if it is a method that takes in &self, all outputs get the lifetime
    //of the self obejct, regardless of how many other input references there are
    //if applying these rules doesn't unambiguously specify the lifetime
    //of the outputs, the compiler throws and error and you have to 
    //specify them 

    //you have to specify lifetimes on struct methods 
    impl<'a> ImportantExcerpt<'a> {
        fn arbitrary_function(&self) -> i32 {
            3
        }
    }
    //you need the first <'a> in order to use it the second time
    //you need the second one because the lifetime is part of 
    //the type of ImportantExcerpt, just like a generic type 
    //annotation

    //there is the 'static lifetime, a special reserved word
    //that means something exists for the entire duration of the 
    //program. all string literals live in the program's binary, 
    //so they are always available, and therefore all have the 
    // 'static lifetime

    






































    






















    







}
