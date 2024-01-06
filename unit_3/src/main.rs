fn main() {

    /*

    ### declaring variables ###

    declare with let
    by default, immutable
    can be overridden with another let (shadowing)

    mutable variables cannot have their type changed

    
    consts cannot be overridden in the same namespace
    consts must have type annotated
    consts cannot be assigned to a value that is computed at runtime
        note: some small operations will be evaluated at compile time
            ex. THREE_HOURS_IN_SECONDS = 3*60*60; is valid

     */

    const A: i32 = 3;
    println!("A is {A}");

    {
        const A: i32 = 4; //this is okay
        println!("A is {A}");
    }
    //here it would not be okay

    //### shadowing ###

    //shadowing is an okay way to override a variable's content while keeping it immutable
    let x = 1;
    let x = x + 1; //still immutable, just has a different value

    //you can declare a new scope and shadow within it
    //then when the scope is over, it reverts to the enclosing definition

    let x = 2;
    {
        let x = 5;
        println!("x is {x}");
    }
    println!("x is {x}");

    /*

    ### Scalar Types ###

    Integers
    unsigned and signed 8 bit ... 128 bit
    i8, u32...etc 

    i32, u32 are defaults

    
    signed variants store -(2^(n-1))...2^(n-1)-1
    
    */
    
    //you can use isize and usize as an int type to grab the default size for your system architecture
    let x: isize = 40;

    //you can also use suffixes to denote types

    let x = 40i32; 

    /*
    you can use _ in numbers to make them more readable, including between
    a number and its type annotation, like x = 40_i32;

    you can write decimals, Hex 0xff, octal 0o77, or binary 0b11

    you can also write specific bytes using b'A', for u8 only

    you can recast on the fly using (x as i32) or something
    note: it may not catch an overflow if you do this

    operations can only happen between objects of the same type

    normally, if the compiler sees an overflow, it makes it so the
    program panics at runtime

    when you compile for release, it makes it so overflow happens
    with two's complement wrapping

    there are special versions of common operations in std that
    define special behavior that relates to overflowing
    there is wrapping_... functions like wrapping_add() that will wrap if overflow

    there is saturating_add(), which saturates a type's max value if 
    overflow were to occur


    Floating Point Types

    f32, f64 (default)

    Bool types

    all lowercase: true, false

    Character types 

    has a char, is a single character in single quotes
    supports all kinds of nontraditional characters, incluing emoji. 
    anything that's in UNICODE

    char in Rust is 4 bytes
    */

    /*

    ### Compound Types

    Tuples

    fixed-lenght sequence of objects of different types

    type declared as 
    let tup: (i32, bool, char) = (40, true, 'a');

    you have tuple unpacking, known as destructuring

    let (x, y, z) = tup;

    you can use .0, .1, .2, ... to access the specific values of a tuple

    an empty tuple is called a unit, and is kind of a special sort of type
    in and of itself. it represents an empty value or an empty return value

    expressions implicitly return the unit value if they don't return 
    any other value. so it's kind of like 'void' or smth

    
    
    Arrays

    fixed length sequences of values of the same type

    allocated on the stack

    declared like 

    let a: [i32; 5] = [3,4,5,6,7,8];
    the second digit is the length of an array

    the std librarry defines a vector, which is a sequence that can change length

    
    */

    let mut a: [i32; 5] = [1,2,3,4,5];
    a[0] = 10;

    /*

    ### Functions ###

    declare like 

    df my_function(params) -> i32 {

    }

    can define functions below main or above; doesnt matter

    you must declare the types of params in function declaration


    Expressions and statemnets
    statements do not return a value. ex. variable assignent
    expressions evaluate to a value

    you can make a little scope that returns a value

    let x = {
        some operations..
        x + 1 //OMIT a semicolon to make the namespace RETURN that value
    }

    the way that a function returns a value in rust is that
    you just put the thign you want to return on the last line 
    with no semicolon. you can also use an explicit return statement


    */

    /*
    Comments

    // single line

    /* 
    multi line
    */

    */


    // ### Control Flow ###

    let x = 5;
    if x == 5 {
        println!("hello")
    } else { //also have else if 
        println!("boo")
    } // no semicolon needed

    //the thing passed to an if MUST be a bool
    //not like python where if 3 will eval to True

    //also have succinct conditional assignment
    let x = if true {5} else {7};

    //in rust you have 3 ways to loop: loop, while, and for

    //a loop loop is an infinite loop that continues until 
    //you break from it. uniquely, it can RETURN A VALUE!!!

    //ex obtain the first multiple of 7 greater than 1234
    let mut x: i32 = 7;
    let result = loop {
        x = x + 7;
        if x > 1234 {
            break x //the value after break is RETURNED by the loop expresson
        }
    };

    println!("result is {result}");

    /*
    you can have LOOP LABELs, a way of labeling your loops 
    to disambiguate between them in break statements

    it's like 

    'loop_1 loop {

        'loop_2 loop {

            if something {
                break 'loop_2
            }

            if something else {
                break 'loop_1
            }

        }

    }
    */

    //while loops self explanatory

    //for loop

    let a: [i32; 5] = [1,2,3,4,5];

    for element in a { //we have this syntax bless tf up
        println!("{element}");
    }

    for number in 0..5 { //special range syntax. so nice wtf
        println!("{number}");
    }
    //note: there are methods you can call on this Range object 0..5,
    //like .rev(), which reverses the range
    //range is NOT inclusive of endpoint unless you put an = before it

    for number in 0..=10 { //special range syntax. so nice wtf
        println!("{number}");
    }


}
