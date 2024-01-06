fn main() {

    // Advanced Rust

    // ### Unsafe Rust ###

    ///unsafe rust allows you to write code that doesnt follow all of rust's
    /// rules. it still has to obey the borrow checker, but it has 5 new powers
    /// 1. ability to derefrence a raw pointer type
    /// 2. call unsafe functions or methods
    /// 3. access or modify a mutable static variable
    /// 4. implement an unsafe trait
    /// 5. access fields of a union S (?)
    /// 
    
    ///for dereferencing a raw pointer, what that means is that there is
    /// this additional type called a raw pointer. it has annotation
    /// *const i32 and *mut i32, respectively. they are like normal
    /// pointers in c++: they may coexist simultaneoulsy, they may point
    /// to null memory, or deallocated memory. you can create them in 
    /// normal rust, but only dereference them in unsafe rust
    //note: the * is part of the name, not the dereference operator
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    //you can also specify that a function is unsafe
    unsafe fn dangerous() {}

    //this could only be called in an unsafe block. a possible use
    //could be if you want to return two slices of the same object
    //as two outputs from a function. rust doesnt know whether 
    //the slices overlap, so it will prevent you from borrowing 
    //aka slicing the thing multiple times. but if you know that
    //the action is safe, you could implement it in an unsafe block
    //inside the function. (see this chap for example)

    ///the same thing applies if we want to set up an interface to call
    /// functions from another language. 
    /// extern "C" {
    ///     fn abs(input: i32) -> i32 {}
    /// }
    /// this would allow us to call the abs() function from C. it is 
    /// unsafe to call functions from other languages, but you could do this
    /// inside an unsafe block
    /// 
    

    ///another use is to modify global static vars. you create these by
    /// static mut MY_COUNTER = 0. it creates a global variable. it can 
    /// be mutable, unlike a const. also, it is delivered from the same 
    /// address in memory each time it is referenced, whereas a const can 
    /// be copied around in the code. it is unsafe to modify these, but you
    /// can do it in unsafe rust.
    /// 
    
    ///you can also implement unsafe traits by 
    /// unsafe trait DangerousTrait {
    /// }
    /// 
    /// then you can do 
    /// unsafe impl DangerousTrait for Blah {
    /// }
    /// 
    /// same idea as unsafe functions and methods
    
    ///the last feature is to access fields of a union, which is a
    /// feature i dont know whose main purpose is to interface with C code

    //### Advanced Traits ###

    ///one feature of traits is associated types. these are kind of
    /// generics, where there is a type associate with a trait whose
    /// name can be used inside the implementation of that trait
    /// 
    /// for example, consider a trait Iterator:
    /// pub trait Iterator {
    ///     type Item;
    /// 
    ///     fn next(&mut self) -> Option<Self::Item> P{
    ///         something
    ///     }
    /// }
    /// 
    /// the Item is a name that can be used to construct the return type 
    /// of the next function. 
    /// if you went to implement the trait, then it would be 
    /// impl Iterator for MyType {
    ///     type Item = u32;
    /// 
    ///     fn next(&mut self) -> Option<Self::Item> {
    ///         ...
    ///     }
    /// }
    /// 
    /// why not just use a generic? if something takes a generic, then
    /// every time you want to call it, you have to specify what 
    /// concrete type to use. here, we dont--each time we call it 
    /// for MyType, Item is assigned to u32 so it is implicit that 
    /// next will return an option<u32>. 
    /// 
    /// 
    
    //one use of associated types would be to implement 
    //the + operator using the Add trait in std::ops::Add

    use std::ops::Add;
    
    #[derive(Debug, PartialEq)]
    //PartialEq allows you to compare equality, necessary for assert
    //also need Debug for the assert call, for some reason
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point; 

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    //note: the def of Add has a type param that just has a defailt
    /// trait Add<Rhs=Self> {
    ///     type Output;
    ///     
    ///     fn add(self, rhs: Rhs) -> Self::Output;
    /// }
    /// Rhs defaults to the type calling the add function 
    /// but could be defined as another type (the thing being added to it)
    /// 
    
    //we can exploit this to implement add for values of different types

    struct Meters(u32);
    struct Millimeters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + other.0 * 1000)
        }
    }
    //now we have specifically defined millimeters + meters

    // ### fully qualified naming ###

    ///one thing that happens is that you can have a method on a 
    /// struct that has the same name as a method inside a trait
    /// that is implemented on it. if this happens, then 
    /// if you just do .method() on the object, then it will 
    /// default to the method native to the struct itself
    /// if you instead want the method from a trait, you do 
    /// Trait::method(&object) note that this fits the function signature
    /// 
    /// note that this only works because the &object will have a type, 
    /// say a Point type, so then the compiler knows you want to call the 
    /// method() as it is implemented for a Point. if however the function 
    /// you want to call is an associasted method that doesnt take the self
    /// as a parameter, then you need to use fully specified syntax
    /// 
    /// <Point as TraitName>::method()
    /// 
    
    //### super traits ###

    ///sometimes you want to implement a trait where you make use
    /// of another trait. the syntax is
    /// 
    /// trait SecondTrait: FirstTrait {
    ///     ...
    /// }
    /// 
    /// now inside the implementation you can call methods on 
    /// objects that are from the FirstTrait 
    /// 
    
    // ### newtype pattern ###

    ///rust has the orphan rule, which says you can't implement 
    /// a trait you didnt define locally on a type you didn't define 
    /// locally. however, you can get around this by making a simple 
    /// Newtype that wraps an instance of the type in a single-element
    /// tuple and then implement the trait for that. this is okay 
    /// because the NewType is local to the project. this actually
    /// has no runtime penalty because the wrapper is removed at compile time
    
    //need to put some content after this to use the ///
    let x = 5;

    //### advanced types ###

    ///you can also use the newtype pattern, where you wrap a struct
    /// of a certain type in a 1-element tuple and make that a new type,
    /// to hide certain aspects of that type's api. for example, 
    /// if you wanted to provide limited functionality on a hashmap,
    /// then you could wrap one in a new type and provide only 
    /// specific methods on it. 
    /// 
    
    ///another thing you can do rather than the newtype pattern is 
    /// type aliasing, which is when you just give a new name to 
    /// an existing type
    
    type Kilometers = u32; 
    ///all uses of u32 are valid uses for Kilometer and vice versa; 
    /// it's literally just a new name. this doesn't afford type 
    /// checking, like in the Meters Millimeters example above, but 
    /// it can save you writing if you have a tedious type, 
    /// like a Box<dyn Trait1 + Trait2 + lifetime>, which is a thing
    /// 
    
    ///Type aliasing is common with Result<T, E>, where E
    /// can often be a complex, niche error type under the hood
    ///the std::io module as functions that all return a std::io::Error 
    /// type, which stands in for all these 
    ///also, the library has an alias for Result<T, E> to save you writing
    /// out all the Es Result<T>
    /// type Result<T> = std::result::Result<T, std::io::Error>;
    /// 
    
    ///there is also a thing called the never type (or in type theory
    /// lingo as an empty type). it is denoted with !. it is a type 
    /// that never returns, and no value can ever be assigned to that 
    /// type. if a function is specified to return it, it means the function
    /// never returns.
    /// fn bar() -> ! {
    /// }
    /// bar never returns
    /// 
    /// the point of it is that sometimes you need a type system to express
    /// the idea that a given function will never return. for example,
    /// in a match statement, all branches need to evaluate to the same
    /// return type. but what if you want a branch to simply continue,
    /// like in the case of continuously getting user input until a 
    /// valid value is entered? 
    /// let guess = match input.trim.parse() {
    ///     Ok(num) => num,
    ///     Err(_) => continue,
    /// }
    /// the only reason this works is that continue formally "returns"
    /// a ! type, so the compiler is like okay the output of this 
    /// match is gonna be a number
    /// 
    
    /// a formal way of saying this is that the ! type can be 
    /// coerced into anything
    /// 
    /// control flow statements like continue, loop (infinite loop 
    /// without a break), and functions that end the program, like panic!
    /// 
    
    //### dynamically sized types ###

    ///some types dont have a size that can be known at runtime, for
    /// example the str type (not &str!!!) which represents a piece of 
    /// text. formally, this is the type that a piece of text is, 
    /// but since you can't know its size until runtime, you can 
    /// never make a variable this type. you have to hide it behind a pointer
    /// &str. you could also do Box<str> or Rx<str>.
    /// the way dynamically sized types work is that a pointer to them 
    /// contains not only the starting position of the object in memory, 
    /// but also the size of it, so that the computer knows where to stop 
    /// reading from. another example of a dynamically sized type
    /// is a Trait, which is a type that has an unknown size (?).
    /// this is why to use "trait objects" as inputs, you have to do like
    /// Box<dyn MyTrait> and hide it in a pointer
    
    ///all that have a known size have a trait under the hood called Sized,
    /// which says that their size is known. implicitly, all uses of a 
    /// generic type have the tacit trait bound that the size of the generic 
    /// type is known, aka that the type implements Sized.
    /// 
    /// fn generic<T>(t: T) {
    /// }
    /// actually means 
    /// fn generic<T: Sized>(t: T) {
    /// }
    /// under the hood. 
    /// 
    /// you can relax this requirement using special syntax
    /// fn generic<T: ?Sized>(t: T) {
    /// }
    /// adding the question mark means that it is okay that the 
    /// generic type may not have known size. this is a special 
    /// trick that only works for the Sized trait
    /// 
    /// 
    
    //### fancy stuff with functions and closures ###

    ///in Rust, you can use not just closures, but also proper functions,
    /// as inputs to other functions. functions have the fn type 
    /// (not to be confused with the Fn trait implemented on closures)
    /// and the way you use them is:
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    ///the above means "f is a function that takes in i32 and returns i32"
    /// note that if you wanted to make a function take in a closure, 
    /// you cant use a type because closures don't have an explicit 
    /// type that you can take in or return--instead, you have to specify 
    /// them using Box<dyn Fn> Box<dyn FnMut> or Box<dyn FnOnce>. 
    ///     (note: these are also your return types if you return a closure)
    /// fn types have all these traits.
    /// in general it is better practice to use the trait bound methods 
    /// so that a function you write can accept functions or closures. 
    /// However, one special case when you might want to accept proper 
    /// functions only is if you are interfacing with a language like C
    /// that does not have closures
    /// 
    
    //an example of doing the same thing with closures and functions

    let list_of_numbers = vec![1,2,3];

    let list_of_strings: Vec<String> = 
        list_of_numbers.iter().map(|x| x.to_string()).collect();
        //note: iter() borrows so we can use list_of_numbers again
    
    let list_of_strings2: Vec<String> = 
        list_of_numbers.iter().map(ToString::to_string).collect();
    
    ///note: when we define an enum, its variants can automatically 
    /// be used to initialize instances of that enum. 
    
    enum Status {
        Value(u32),
        Stop
    }

    let list_of_statuses: Vec<Status> = (0 as u32..20).map(Status::Value).collect();

    
    //### Macros ###

    ///implementing macros is a complex topic i will only summarize
    /// for my own reference later. one needs to read the doc to learn it 
    /// thoroughly. 
    /// 
    /// the main idea is that macros are different from functions because
    /// they replace code with other code at compile-time. they have the 
    /// advantage that they can take variable  numbers of arguments, because
    /// functions cant. 
    /// 
    /// macros must be defined before their use in a program
    /// 
    /// there are two types of macros: declarative macros and procedural 
    /// macros.
    /// 
    /// declarative macros take in some code, match it against other code
    /// using something similar to the idea of a regex, and then return other code
    /// an example is the vec! macro; it takes in a sequence, matches it 
    /// to a regex, and then generates a vec and pushes each element to it
    /// based on how many times the pattern that was designed to match against
    /// each element did so. this is what gives vec! the power to take in 
    /// variable length/type arguments. 
    /// 
    /// you make declarative macros using the macro_rules! macro
    /// 
    /// there is info abotu macro patterns the the matching expression
    /// syntax in the little book of rust macros and rust reference, respectively
    /// 
    /// the other kind of macros is procedural macros. they come in three
    /// types: 
    ///     custom derive
    ///         allows you to write #[derive(MyCustomTrait)], so you 
    ///         can implement your custom trait on different structs
    ///         and enums using this syntax instead of having to 
    ///         define everything in an impl block
    ///     
    ///     attribute-like
    ///         allows you to write your own attributes,
    ///         which are the things in the #[] statement, such as derive
    ///         examples would be to implement things that behave
    ///         like attributes but can be used above functions (normal
    ///         attributes can only be used on structs and enums). 
    ///         this would enable you to implement something like a 
    ///         function decorator in python to use annotations like
    ///         in Flask
    /// 
    ///     function-like
    ///         these are similar to declarative in that they take in some
    ///         code and return some code, but they do more complex things 
    ///         than matching. an example would be the sql! macro, which takes 
    ///         in raw SQL and then can either tell you if it's syntactically valid
    ///         or execute it. this is more complex than regex pattern matching; 
    ///         it's like full parsing
    /// 
    /// for implementing procedural macros, you have to implement them in
    /// a separate project from where you want to use them and structure the 
    /// directory in a specific way. see the reference for info



    

















    

















}
