
enum IPAddressKind {
    V4, //these values don't need to refer to a kind of var that already exists. theyre just raw
    V6
}

enum IPAddress {
    V4(String),
    V6(String),
}

enum IPAddress_2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32}, //this behaves like a struct; it has named fields
    Write(String),
    ChangeColor(i32, i32, i32),

}

impl Message {
    fn call(&self) { //takes  reference to self, equiv to self: &Self
        //do something
        //how do you, say, print your own value?
        //you have to implement a formatting method
    }
}

enum Coin {
    Penny,
    Nickel, 
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1, //note use of =>
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25, //these numbers are expressions
        //the value each expression returns is returned by the entire
        //match expression, which is then returned by the function
    }
    //you can use curly braces or not if the expression is one line,
    //but if you want to run multiple lines, you must use curly braces
}


enum StateCoin {
    Penny, 
    Nickel,
    Dime, 
    Quarter(String)
}

fn value_in_cents_with_state(coin: StateCoin) -> u8 {
    match coin {
        StateCoin::Penny => {
            println!("Lucky penny!");
            1
        },
        StateCoin::Nickel => 5,
        StateCoin::Dime => 10,
        StateCoin::Quarter(state) => { //pulls out the state from Quarter and binds to state
            println!("Quarter from {}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { // if output could be None, has to be Option
    match x {
        None => None,
        Some(i) => Some(i + 1) //the output has to be a some ?
    }
}


fn main() {
    /*

    ### Enums ###

    enums are a way to say that a variable is one of a set of values
    the values it is allowed to be are called variants

    the variables type remains the base type of the enum, not the variant

    imagine we have a type that represents an IP address. there are only
    two protocols for that that are in use. V4 and V6. a given variable
    can only be one of them. so we use an enum to encode this:
    
    defined above

    you can put data inside a variant
    like we have two poss values for the ip address type, 
    but what about storing the addresses themselves?

    the way you speak of this is that you say 
    the variable is one of the possible variants AND that 
    it stores some information
    
    */

    let four: IPAddressKind = IPAddressKind::V4;
    let six: IPAddressKind = IPAddressKind::V6;
    //values an enum can take are found using the path notation
    //under its name
    //both the vars four and six are IPAddressKind type


    //example of having enums with value in the variants

    let home: IPAddress = IPAddress::V4(String::from("127.0.0.1"));
    let loopback: IPAddress = IPAddress::V6(String::from("::1"));
    //you still use :: to access variants and then parens to assign values

    //IPAddress::V4() is a function that constructs an instance of the
    //IPAddress type in the V4 variant

    //you can store Different amounts of data in different variants 
    //of an enum type

    let home = IPAddress_2::V4(127, 0, 0, 1);
    let loopback = IPAddress_2::V6(String::from("::1"));

    //you can also make the data stored by an 
    //enum variant a struct

    //let's define a message enum above
    //note that the values that the variants store are complex;
    //if you wanted to implement the same thing with structs,
    //you'd have to define four kinds with different fields
    //but then they wouldn't have the same type, whereas with enum
    //all the variants are a Message type. this is why enums are superior


    //we can also define methods on enums, see above

    /*

    ### the Option type ###

    the option type encodes when a value could be something or it could be nothing
    Option is defined in the standard library

    for example, if you access an element from a list, 
    then you could either access a valid element or get something
    or you could access an index that does exist and get nothing
    this is a scenario where to have truly safe code, you'd need to 
    error handle, and the Option type could help you do this 
    by encoding the reality that you either get somethign or you get nothing
    and the compiler could make sure you have handled it

    also Rust does not have a null pointer. you only have the 
    Option enum, which could be something or it could be nothing

    so rust implements the 'null' as 
    enum Option<T> {
        None,
        Some(T),
    }

    this is such a common feature it's in the prelude,
    so you don't have to bring it into scope
    you can also use the variants without doing :: 
    because theyre in the prelude

    the <T> is a Generic type, and it means that Some()
    takes in a generic paramter of any type, and then the Some()
    BECOMES that type

    so we have:

    */
    let some_number = Some(5); //here rust infers the type from the input
    let come_char = Some('e');

    let absent_number: Option<i32> = None;
    //a number that we can't reference or is absent but should
    //be i32 if it existed
    //we need to declare the type bc rust can't infer anything from "None"

    /*

    one key thing is that when you do something like 
    let some_number = Some(5); its type is Option<i32>
    it is not the same thing as a true i32. this means you cannot
    perform operations with it because operations are not 
    supported between i32 and Option<i32>. 

    you need to write code that explicitly handles the possibility 
    that the Option<i32> will be None and extract the <i32> from it 
    if it is there so that it can be used for operations

    the idea is that the compiler forces you to handle all the cases
    where the value could be null so that you never run into an 
    issue with trying to do something with a null that shouldn't be done

    there are many methods for an Option that extract its value
    
    */

    /*

    The Match Control Flow Construct

    match allows you to match an expression against a bunch of 
    patterns. there are lots of ways to define the pattern to match with

    inputs are mapped to the FIRST thing they pair with

    let's implement an enum for coins and write a function 
    that takes it in and returns the number of cents associated with it


    Match statements can also bind to values
    in an enum, if a variant holds data, you can reference it 
    in the match statement Variant(variable). the value stored
    in the variant will bind to the 'variable' variable in that 
    code block and you can use it.

    example: 
    make a new type of coin that puts a state in each quarter
    extract and bind that state variable in the match statement 
    for quarter


    Matching with Option<T>
    the whole idea of the Option<T> is that it forces you to 
    handle the case when a value could be null every time you use it 
    because Option<T> is not the same type as T. The match statement makes
    it easy to handle this

    #let's return to the idea of the Option<i32> type from before
    and write a function that adds one to such a value. the match
    statement has to handle the case when the value is None
    
    //Match statements must cover every possibility 
    there are two catch-all values you can use if you want to implement
    default behavior for all branches of a match statement that arent
    specified. one is 'other', which DOES bind a value, and the other is 
    '_', which does not bind a value. the purpose of the latter is to tell
    Rust that we aren't going to use the value, so it won't warn us about 
    an unused enum variant


    if let syntax

    sometimes you want to do something if an enum is one value and 
    do something else if it is any other value. one way to do this
    would be a match statement with a _, but that can be verbose.
    another way is to use the if let syntax. 

    if let variant(value) = my_enum {
        do something with value 
    }

    note: this thing here is checking if my_enum is in the desired
    VARIANT, not whether it is carrying a desired value. value is just
    a dummy variable that binds to whatever is inside my_enum

    the way to think of this is like it's saying that the assignment
    (variant(value) = my_enum) only "works" if my_enum is in the desired
    variant, so in that sense it has a bool answer associated with it
    this is not really what's happening under the hood, necessarily; 
    this is a standalone syntactic sugar

    you can also handle the else aka _ case in an if let

    if let variant(value) = my_enum {
        do something with value
    } else {
        do something else with no value
    }

    is exactly equivalent to 
    match my_enum {
        variant(value) => {
            do something with value
        },
        _ => {
            do something else with no value
        }
    }

    
    
    */

    let my_coin: Coin = Coin::Dime;
    let value: u8 = value_in_cents(my_coin);
    println!("value is {}", value);


    let special_coin: StateCoin = StateCoin::Quarter(String::from("Alaska"));
    let value: u8 = value_in_cents_with_state(special_coin);
    println!("the value is {}", value);



    let option_1: Option<i32> = Some(5);
    let res1 = plus_one(option_1); //6











    











    
}

