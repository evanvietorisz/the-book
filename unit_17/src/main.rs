fn main() {
    //Object Oriented Programming in Rust

    // ### Characteristics of OOP languages ###

    /// Rust has features that in some ways map onto the features associasted
    /// with object oriented programming languages
    
    /// 1) Objects. Rust has objects insofar as it has enums and structs
    /// and the ability to write impl blocks on them to define methods
    /// on the data they contain
    
    /// 2) Encapsulation. Rust provudes encapsulation in that you 
    /// can use the pub keyword to control whether code using your code
    /// has access to certain methods and fields of structs.
    
    /// 3) Polymorphism. this is the idea of a function being able
    /// to operate on inputs of differnet types. using generics and 
    /// trait bounds, Rust allows you to write code that can work
    /// with multiple types and provide requirements on what those 
    /// types are able to do
    
    /// 4) inheritance. inheritance is messy because sometimes you don't 
    /// want your code to inherit everything from the parent class, as some
    /// of it may not apply and can lead to a bloated class. Rust implements
    /// this idea using trait objects; rather than specify multiple functions
    /// to do the same thing on different types, you implement types that have
    /// certain abilities or traits, so that other code can do stuff with them
    /// flexibly. 
    
    // note: there is no traditional inheritance in rust


    //let's do an example of encapsulation for the sake of it:

    //let's make a struct that holds a list,
    //allows you to add and remove from it, and 
    //automatically maintains its average

    pub struct AveragedCollection {
        list: Vec<i32>,
        average: Option<f64>,
    }

    impl AveragedCollection {

        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: vec![],
                average: None,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result: Option<i32> = self.list.pop();

            match result {
                Some(val) => {
                    self.update_average();
                    Some(val)
                },
                None => None,
            }
        }

        pub fn average(&self) -> Option<f64> {
            self.average
        }

        fn update_average(&mut self) {
            if self.list.len() == 0 {
                self.average = None;
            } else {
                let total: i32 = self.list.iter().sum();
                let avg = total as f64 / self.list.len() as f64;
                self.average = Some(avg);
            }
        } 
    }

    let mut ac = AveragedCollection::new();
    println!("current avg is {:?}", ac.average());

    ac.add(1);
    ac.add(2);
    ac.add(3);
    println!("current avg is {:?}", ac.average());

    ac.remove(); //ah so it is possible not to assign the output of a function
    println!("current avg is {:?}", ac.average());


    //### Using Trait Objects That Allow for Values of Different Types

    ///Rust doesn't have inheritance, but it does have trait objects,
    /// which allow you to implemenet certain functionality on 
    /// different types you create and write code that is agnostic 
    /// to type but ensures that the inputs it accepts have certain
    /// necessary functionality
    
    ///say we want to build a gui library where all the objects
    /// (some of which a user would implement themself) need to 
    /// have a draw method. we do this by defining a draw trait,
    /// which defines a draw method, and a user would implement
    /// this trait for every custom type they define

    /// to do this, we implement a trait object, a thing that stands in 
    /// for your type in your code. the idea is that it's a pointer to some
    /// object that implements the specified trait. the syntax is
    /// Box<dyn Trait> 
    
    pub trait Draw { //this trait exists
        fn draw(&self); //it entails a function of this signature called draw
    } //all types that implement Draw need to implement this function

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() { //why don't need a &
                component.draw();
            }
        }
    }

    ///note: this is different than using a generic type bc a generic
    /// type can only be substituted for one concrete type at runtime
    /// for example:
    
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }

    // impl<T> Screen<T>
    // where
    //     T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }

    ///would only work if T were a single type a runtime. a Screen
    /// would have to contain all one of the same type
    
    ///if we have the Box<dyn Draw> syntax, then we can substitute it 
    /// for any number of types that implement the Draw trait and have 
    /// a heterogeneous collection at runtime
    /// 
    

    //example

    struct Button {
        height: u32,
        width: u32,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("The draw method for Button");
        }
        
    }

    struct SelectBox {
        param1: u32, 
        param2: u32,
        param3: u32,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("The draw function for SelectBox")
        }
    }

    let s = Screen {
        components: vec![
            Box::new(Button { //need to wrap each object in a box
                height: 5,    //bc components is Vec<Box <dyn Draw>>
                width: 4,
            }),
            Box::new(SelectBox {
                param1: 1,
                param2: 2,
                param3: 3,
            }),
        ],
    };

    /// note: when you use a generic type, the compiler swaps it 
    /// with a concerete type at compile time, leading to static
    /// dispatch, which is where the compiler knows what methods are 
    /// being called a compile time. when you use Box<dyn Trait>, 
    /// the compiler doesn't know exactly what method is being used
    /// at compiletime because it hasn't been implemented yet by the 
    /// user of the library, so it compiles code to do dynamic dispatch,
    /// which is where the program figures out what method to call at runtime.
    /// this carries a cost at runtime and precludes inlining the method being called
    

    // ### Implementing the State Pattern ###

    ///omitting this chapter because it is tedious and not that interesting
    /// basic idea is that if you want a stuct that can only have certain
    /// methods performed on it in differnet orders, then you can implement
    /// an underlying state for it to have. that state will be a custom
    /// structs which all implement a State trait. we make that trait
    /// hold methods which enact all the possible state transitions. 
    /// then we make it so that only a desired state can actually access
    /// certain fields in the main struct so as to return special content 
    /// at certain times
    /// .
    /// this is tedious, and it makes it so that all structs that implement
    /// the State trait must implement EVERY method that any struct could use
    /// and just do it so that it does nothing when you dont want it to yield
    /// an effect. something better would be to make it so that incorrect
    /// method calls arent even possible. you could do this in rust by 
    /// making the states types, not structs. then each type has only 
    /// certain methods. you enact state transitions by taking ownership of the 
    /// current state and returning a new state of a new type that enables new 
    /// actions








    









    

    




    




    







}
