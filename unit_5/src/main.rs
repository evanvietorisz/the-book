fn main() {

    /*

    ### Structs ###

    like the data attributes of an object in an object oriented language

    */

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    //initiate
    let my_user = User {
        active: true,
        username: String::from("bigdaddy420"),
        email: String::from("johnappleseed@gmail.com"),
        sign_in_count: 1,
    }; 
    //access an attribute with . notation
    println!("{}", my_user.username);

    //build a struct from a function
    let mut my_second_user = build_user(String::from("joe"), String::from("joe@gmail.com"));

    my_second_user.username = String::from("JointRoller3000");

    println!("{}", my_second_user.username);


    //we can initiate a struct using the params of another, 
    //only modifying the values we choose

    let my_third_user = User {
        username: String::from("Splooj_master_69"),
        ..my_second_user //grabs all remaining parameters from my_second_user
    };//NOTE THIS MOVES my_second_user into my_third_user. those strings are 
    //not longer owned my my_second_user
    //if we had created new fields for BOTH strings, then everything else
    //would be copied and my_second_user would still be valid


    //note: in general, you want a struct to OWN all its heap-allocated data
    //if you want it to REFERENCE some data, then you need lifetimes,
    //which ensure that that data lives as long as the objects that reference it 

    fn build_user(username: String, email: String) -> User {
        User { //note: to know about this Struct type, the function needs to be within the namespace where it was defined
            active: true,
            username, // equivalent to username: username, 
            email, // this is called Field init shorthand
            sign_in_count: 1
        }
    }


    //Tuple Structs

    struct Color(i32, i32, i32); // declare

    let my_color = Color(0,0,0);
    //functions that take in a Color cannot take in any old tuple of 3 i32s

    //Unit like structs
    //similar to () tuple, structs with no data. 
    //you can implement traits on them

    struct UnitStruct; 
    let x = UnitStruct; //x is an instance of UnitStruct


    //### adding functionality to structs

    //say we had a struct called rectangle and wanted to be able 
    //to print it. we cant just use the normal method
    //because we haven't implemented the functionality to 
    //represent the struct, and it doesnt come with it.
    //but we can imbue the struct with DEBUG functioanlity,
    //the ability to render a simple representation 
    //when we call to it using the {:?} syntax in the 
    //formatted string. this syntax calls the debug representation,
    //as opposed to the normal once called by {}

    #[derive(Debug)] //adds debug Trait, like a decorator??
    struct Rectangle {
        height: u32,
        width: u32,
    }//note: the derive term is an attribute--not sure what that means yet

    //note: Debug is a TRAIT, just like the COPY Trait

    let my_rect = Rectangle {
        height: 30,
        width: 20,
    };

    println!("{:?}", my_rect); //{:#?} also works, and is prettier

    //note: we could also use the dbg! macro to check what
    //things are evaluating to
    //it tells you what an expression evals to, returns
    //ownership of the result

    let x = dbg!(3 * 9);
    
    //can also use it to see a whole object
    //since it takes ownership and returns a result, 
    //but rectangle has no result, then we want to 
    //pass in a reference to a rectangle
    //so debug can display it without taking ownership

    dbg!(&my_rect);


    //### methods ###

    /*

    methods go inside impl (implementation) blocks
    inside these blocks go all functions associated with a struct

    within this block, to define a method, you define 
    a function like any other. to be a method, 
    that function must take in a first parameter called self
    that is type 1) just an instance of that type itself (so it consumes the object)
    2) takes a reference to it (borrows)
    3) takes a mutable reference to it

    so it would be like 
    fn my_method(self: Self) {
        ...
    }
    fn my_method(self: &Self) {
        ...
    }
    or 
    fn my_method(self: &mut Self) {
        ...
    }

    note: within the impl block, the word Self is an alias for the 
    type that the impl block is associated with. it is literally just another 
    way to write it

    Rust lets you abbreviate the above as
     fn my_method(self) {
        ...
    }
    fn my_method(&self) {
        ...
    }
    or 
    fn my_method(&mut self) {
        ...
    }

    it means the exact same thing as above, and is just easier to write

    the idea of moving, borrowing, or mutable borrowing is the 
    same as in any other circumstance. taking in a reference 
    is prob the most common use. to take in the object itself, 
    you'd have to want to transform it, return somethign new, and 
    never use the original object again

    in c++, you use . to access an object's method and -> 
    to dereference an object pointer and then call its method

    in Rust, rather than write ->, you have *automatic dereferencing*,
    where if you use . on the object or a pointer to it, Rust will
    automatically figure out what you need and create the appropriate
    reference to it. like it will make, say, the &mut Object reference 
    for you under the hood and pass it into the method you call

    note: there can be multiple impl blocks, though there's no 
    reason for there to be in short scripts. 

    you can have a method and an attribute of the same name;
    you just call the method with parentheses and the attribute with none

    
    you can also define functions associated with a type that are not 
    methods on it. these functions don't take in self as a parameter
    one example of them is a constructor. it's not a method, 
    but it's like associated with the type. String::new() is a common 
    example -- note: new() is not a reserved word, just a popular choice 
    for constructors. we access associated functions which are not methods
    using the :: path operator

    */

    impl Rectangle { // everything in here is associated with Rectangle type
        fn area(&self) -> u32 {//takes in reference to self, aka self: &Self
            self.width * self.height
        }

        //make an associated function to create a new 
        //type of rectangle with both sides same dimension
        fn square(side_length: u32) -> Self { //returns a Rectangle instance
            Self {
                width: side_length,
                height: side_length,
            }
        }

    }

    let my_square = Rectangle::square(4);
    println!("{:?}", my_square);








}

