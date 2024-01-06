
enum List {
    Cons(i32, Box<List>), //32 bit integer, pointer to List
    Nil,
} //boxes are just addresses, so rust knows how large they are

fn main() {
    /*

    ### Smart Pointers ###

    Pointers are variables that contain an address in memory
    Rust references are pointers. they have no overhead or metadata

    Smart pointers are pointers with additional metadata

    smart pointers exist in other languages too


    in Rust, smart pointers often own the data they point to, while
    references only borrow it

    Strings and Vec<T> are types of smart pointers because they point to 
    some memory that they OWN, allow you to manipulate it, and have 
    some metadata

    for example, a String stores its capacity and has the ability to 
    guarantee its data is a valid UTF-8 encoding 

    Smart pointers are implemented as structs that have two special traits
    the Deref trait, which allows the struct to behave like a reference
    the Drop trait, which allows you to customize what happens when an 
        instance of the type goes out of scope


    
    
    ### Using Box<T> to point to data on the heap ###

    Box<T> is the most basic type of smart pointer that allows you 
    to point to data on the heap. they have no overhead other than 
    dereferencing. you use them when you:
        - have a type whose size can't be known at compile time
        and you want to use it later in a context that requires an exact size
        - when you have a large amount of data and want to transfer ownership
        and want to ensure it won't be copied when you do 
        - when you want to own a value and you only care that it implements 
        a specific trait rather than be a specific type

    
    so that means transferring ownership copies data on the stack
    
    */

    //using Box to store data on the heap

    //create a box
    let b = Box::new(5);
    println!("my box contains {}", b);
    //b is dropped automatically when b goes out of scope


    //recursive types with boxes

    //a recursive type is a type that contains fields which are 
    //the same type as it. an example is a 'cons list' from Lisp
    //(3, (4, (5, Nil))). each entry is a Pair, and they're nested 
    //recursively. this is Lisp's version of a linked list. 

    //since this is recursively defined, you can't know 
    //how big it will be at compile time. so it could never 
    //be allocated on the stack. so the Box allows us to 
    //allocate it on the Heap

    //let's define it
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    //this gets an error "infinite size. use a box to break the recursion"
    //because rust is cant figure out how much space this takes 
    //on the stack bc of the recursive definition

    //look at how rust figures out how much space is taken up by a nonrecursive type

    enum Message {
        Quit,
        Move { x: i32, y: i32 }, //named fields
        Write(String), 
        ChangeColor(i32, i32, i32), //just contains unnamed values
    }
    //an enum is only ever one of its variants at a time, so 
    //rust looks through these variants and says "the size of
    //Message = the size of the largest variant it could possibly be"

    //so when you define a recursive type, there's an infinite 
    //loop, and rust is like okay i cant figure out how big this is
    //supposed to be. thats why you need to allocate it on the heap

    //following the compiler message, we change the data structure to say

    //defined above
    // enum List {
    //     Cons(i32, Box<List>), //32 bit integer, pointer to List
    //     Nil,
    // } //boxes are just addresses, so rust knows how large they are

    //so now we can create a List like so:

    use crate::List::{self, Cons, Nil};//bring enum into scope from outside

    let list:List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    
    //### Treating smart pointers like references with the Deref trait ###

    //allows you to cusotmize the behavior of the dereference operator *
    //allows you to make it so a smart pointer behaves like a reference, 
    //so that code that works on references can also work on smart pointers

    //let's review how dereferencing works
    let x: i32 = 5; 
    let y: &i32 = &x; 

    println!("y is {}", y); //automatically gets "dereferenced"

    //assert_eq!(x, y); //but this does not work bc you can't compare int and &int

    //but this does:
    assert_eq!(x, *y);

    //we can do the same basic thing with a box

    let x: i32 = 5;
    let y: Box<i32> = Box::new(x); //points to a copied value of x allocated on the heap

    assert_eq!(x, *y);

    //let's implement something that behaves similarly to a Box
    //to see how default smart pointers behave differently 
    //from references. then we'll implement the Deref trait on it
    //to enable it to be dereferenced like a reference

    struct MyBox<T>(T); //is a tuple struct; no named fields, just holds a value
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    //let's redo the code from before using MyBox

    //note: here we're not gonna actuall implement heap allocation; 
    //this is just an example of making a thing dereference-able

    // let x = 5;
    // let y = MyBox(x);

    // assert_eq!(x, *y);
    //this will not work because currently we can't dereference MyBox
    //we need to implement the deref trait


    use std::ops::Deref; 

    impl<T> Deref for MyBox<T> {
        type Target = T; //declares an associated type. basically is just another way of declaring a generic parameter
        
        //this is what happens when you dereference a MyBox
        fn deref(&self) -> &Self::Target {
            &self.0 //return a reference to the 0th element of this tuple struct
        }
    }
    
    //now we can do:
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y);
    //now when we write *y, what actually happens behind the scenes
    //is *(y.deref()), which evals to *(a reference to the 0th element in y)
    //this makes clear that the deref method needs to return a reference to something
    //because references are what can be dereferenced

    //### Deref Coersion ###

    //deref coersion is the feature that if a certain pointer points to an
    //object of a certain type, if you dereference it, it can return a reference
    //to an object of a different type. for example, a thing that points to a 
    //String can be automatically converted to a &str

    //deref coersion happens on any type that implements the Deref trait

    //example:

    //this function requires a string slice &str

    fn hello(name: &str) {
        println!("hello, {}", name);
    }

    //but we can also call it on a pointer to a String

    let my_name: MyBox<String> = MyBox::new(String::from("Evan"));
    hello(&my_name);

    //see how that just worked? Since MyBox has the Deref trait, when
    //we generate a reference to it, it is automatically converted from 
    //a reference to a String to a &str.

    //if we didn't have the feature, we'd have to do
    hello(&(*my_name)[..]);
    //dereferences my_name, creates a reference to it, then creates a 
    //slice of it containing all the content
    
    //there is no runtime penalty for using deref coersion bc figuring 
    //out what needs to happen occurs at compile time

    //How Deref coersion interacts with mutability
    //just as Deref overwrites * on immutable references,
    //the trait DerefMut allows you to overwrite the * operator
    //on mutable references

    //deref coersion can convert from immutable to mutable,
    //mutable to mutable, and mutable to immutable

    //CANNOT do immutable to mutable


    //### The Drop Trait ###

    //Drop trait customizes what happens when a smart pointer goes
    //out of scope. common things are deallocating heap space
    //and closing a network connection

    //ex

    struct CustomSmartPointer {
        data: String,
    }

    //the Drop trait requires that you implement a function called drop
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("drop() was called on smart pointer with data '{}'", self.data);
        }
    }//we don't need to bring the Drop trait into scope

    //this will be called whenever a variable goes out of scope
    let c = CustomSmartPointer {
        data: String::from("first object"),
    };

    let d = CustomSmartPointer {
        data: String::from("second object"),
    };

    //things are dropped in reverse order from how they are created

    //you can't call an object's drop method manually. if you want
    //to drop it early, you have to use 

    use std::mem::drop;

    let x = CustomSmartPointer {
        data: String::from("some more stuff"),
    };

    drop(x);//x goes out of scope here
    //a common use case would be if there is a lock involved and 
    //you want to free it up for some reason

    //you actually don't need the std::mem... line; drop() is in the 
    //prelude, so you can just call it



    //### Rc<T> ###

    //Rc<T> is a reference counted smart pointer implemented in the 
    //standard library

    //Rc<T> enables multiple ownership. you use it when you have
    //multiple users of a single object and can't tell at compile time
    //who will be using it last

    //Rc<T> is only for single-threaded situations

    //let's use Rc<T> to create a kind of linked list where two 
    //different heads point to the same end of a list, like a converging
    //fork in the road

    //we have to reimplement the List type using 

    enum RcList {
        RcCons(i32, Rc<RcList>),
        RcNil,
    }

    use RcList::{RcCons, RcNil};
    use std::rc::Rc;//can be pointed to / owned by multiple things

    //all the Rcs are just to avoid name collisions with 
    //the List we defined previously

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    //5 -> 10 -> Nil

    let b = RcCons(4, Rc::clone(&a));
    let c = RcCons(3, Rc::clone(&a));
    //b and c are RcLists that have a value and now point to 
    //a cloned (copied) instance of the address of a
    //a is aware that both b and c point to it, and will only
    //be deallocated when both b and c are deallocated

    //note: we could have just used a.clone(), but this is slower 
    //than the custom, special Rc::clone() function because it makes 
    //a deep copy, whereas the Rc::clone() just copies the address

    //let's explicitly check the ref counts
    //a exists

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));

    println!("ref count on a is {}", Rc::strong_count(&a));

    let b = RcCons(4, Rc::clone(&a));

    println!("ref count on a is {}", Rc::strong_count(&a));

    {
        let c = RcCons(3, Rc::clone(&a));
        println!("ref count on a is {}", Rc::strong_count(&a));
    }
    println!("ref count on a is {}", Rc::strong_count(&a));

    //Rc<T> only allows immutable borrows
    //the next section shows you how to get around that


    //### RefCell<T> ###

    /*
    Interior mutability is a design pattern that allows you to 
    change an immutable object, which is usually against the rules.
    it uses unsafe code inside a special data structure to get around 
    normal borrowing rules

    unsafe code means that to write the code the normal compiler rules
    were lifted, a human made sure the code was not prone to errors, and
    now it's allowed
    */

    //the RefCell<T> type follows the interior mutability pattern

    //the RefCell<T> type only allows a single owner, unlike Rc<T>,
    //but its special property is that its borrowing rules are enforced
    //at runtime, not compile time. if an error occurs, the program
    //panics and exits

    //the advantage of this is that maybe there is a program you know is correct
    //but the compiler can't be sure--it happens

    //the RefCell<T> type allows you to implement the thing you want 
    //while the compiler would prohibit you from doing it

    //like Rc<T>, RefCell<T> is only for single-threaded contexts

    //beause the borrow rules fro RefCell<T> are enforced at runtime, 
    //you can do a mutable borrow to an immutable value

    // let x = 5;
    // let y = &mut x; 
    //this is not allowed bc you can't borrow an immutable object as mutable

    //but what if you wanted to create an object that was able to mutate
    //itself internally but appear immutable to all external code?
    //RefCell<T> is the answer

    
    //a use case for this is Mock objects, objects whose purpose is to be
    //used in place of other ones and record what happens to them for the 
    //purpose of testing code. 

    //let's implement one

    //let's say we're building functoinality that keeps track of something
    //like api calls. we want to limit it and will create a class called
    //a limit tracker. it will take as a parameter a messenger object,
    //whose purpose is to send a message to a particular channel
    //phone, emial, etc based on the actions performed by the limit tracker
    //the mock object will replace the messenger and simply 
    //write all the messages that would be sent to some internal data structure

    //see lib.rs

    //RefCell<T> exposes a safe API with borrow() and borrow_mut()
    //methods that you use instead of creating references using 
    //& and &mut. these methods return Rev<T> and RefMut<T> types.

    //the object keeps track of them number of references at runtime
    //and panics if the rules are violated

    // a common use of Rc and RefCell is to combine them 
    //to have an Rc that contains a RefCell so that there
    //can be multiple owners of mutable data

    //quick example, modifying our List thing from before
    //use std::rc::Rc; //already defined above
    use std::cell::RefCell;

    #[derive(Debug)]
    enum SharedList {
        SharedCons(Rc<RefCell<i32>>, Rc<SharedList>),
        SharedNil,
    }

    use SharedList::{SharedCons, SharedNil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(SharedCons(Rc::clone(&value), Rc::new(SharedNil)));

    let b = SharedCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = SharedCons(Rc::new(RefCell::new(5)), Rc::clone(&a));

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    //now let's change the value in a
    //dereference the Rc to get the RefCell, borrow as mut to change
    *value.borrow_mut() += 10;

    // *value gives RefCell, .borrow_mut() gives RefMut<T> type,
    //which contains a pointer to the integer. it is 
    //AUTOMATICALLY dereferenced when we do += 10. 
    //no need for the -> operator as in C++

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);


    //### Reference cycles can leak memory ###

    //if you use Rc and RefCell to create a cycle, like
    //two linked list nodes that point to each other, 
    //then their ref counts may never go to one and the 
    //memory may never be dropped

    //one way around this is to replace some relationships
    //in a network with non-ownership relationships:

    //you use Rc::downgrade, which creates a weak relationship
    //this creates a pointer that dosnt add to an Rc's .strong_count,
    //it increases the weak_count. this can be nonzero--
    //whenever the strong count reaches zero, the object gets cleaned up
    //the resulting smart pointer is a Weak<T> type

    //Weak references can be dropped at any time if the strong_count
    //hits zero, so to access the thing inside a weak reference, you 
    //use the upgrade method and have to CHECK if the thing that it 
    //originally pointed to is still there. thus, the method 
    //returns an Option<Rc<T>>

    //let's demonstrate this by creating a tree structure
    //notes will be aware of their children and their parents

    use std::rc::Weak;
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    } //Node owns its children
    //we want to modify which nodes can be children of another node,
    //so we have a refcell

    let leaf = Rc::new(
        Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()), //empty Weak pointer
        }
    );//a node can refer to its parent but doesnt own it

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //use the borrow to borrow the RefCell, upgrade()
    //to refer to the thing pointed to by the Weak reference

    let branch = Rc::new(
        Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        }
    );

    //make branch leaf's parent

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    //assign the content inside the RefCell to a weak reference to branch
    //the downgrade creates the weak reference

    //     3
    //   /
    // 5

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    








































































    













































}
