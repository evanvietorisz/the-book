use std::process; 

fn main() {

    /*

    ### Closures ###

    anonymous functions that you can pass around and which can capture 
    values from the scope they're defined in (unlike functions which
    can only look at arguments passed in as local variables)

    an exampleof a closure is the unwrap_or_else function. it 
    'unwraps' an Option<T> and if it is a Some(T) returns the T, else
    calls a closure you pass into it as input
    */

    let x: Option<i32> = Some(4);
    let x: Option<i32> = None;

    let y = x.unwrap_or_else(|| return 10);

    println!("{y}");

    let x: Result<i32, &'static str> = Ok(5);
    let w: i32 = x.unwrap_or_else(|err| {
        println!("there was an error: {err}");
        process::exit(1);
    }); //idea is that x could also be Err("the string slice in the error varianet")


    //closure params go in the || at the beginning
    //if you have statements and not expressions, you need to put the whiole
    //closure in {}

    //most of the time closures don't require type annotations
    //because they don't expose an interface and so you don't need 
    //to enforce agreement between different parties on what the 
    //types of the arguments. closures are usually used in one 
    //little local context, so the types can be inferred. if they
    //can't, the compiler will let you now and force you to annotate them

    //you can annotate if you want
    let my_closure = |num: i32| -> i32 {
        println!("returning  number {}", num);
        num
    }; //see? you can just assign function to a variable

    //note: if you define a closure and don't use it ever, the compiler
    //will complain because it needs a use case to infer types from
    //types are inferred from the first time the closure is used

    //closures can borrow immutably, mutably, and take ownership
    //by default, they borrow immutably

    let v = vec![1,2,3];
    let borrows_immutably = || println!("borrowed immutably: {:?}", v);
    //note: no {} needed bc closure fit on one line 

    borrows_immutably();

    //it can also borrow immutably
    let mut v = vec![1,2,3];
    let mut borrows_mutably = || v.push(10);
    //if you want to borrow mutably, you need to put mut on the closure

    //note: the mutable reference exists at the DEFINITION, 
    //so you couldn't, say, print the vec here because that would 
    //be a borrow overlapping with a mutable borrow

    //the reason the borrow starts here is because you didn't pass
    //in by reference; you captured the scope 

    borrows_mutably();
    println!("vec after calling borrows_mutably: {:?}", v);

    //or you can force a closure to take ownership with the move keyword

    let v = vec![4,5,6];
    let takes_ownership = move || println!("we OWN this {:?}", v);

    takes_ownership();

    //println!("the vec: {:?}", v); would not work

    //a common use of moving in closures is to pass something to a new thread

    //closures implement the FnOnce, FnMut, Fn traits in an additive way
    //closures that have FnOne means  can be called once
    //FnMut means they can be called multiple times and mutate their arguments
    //Fn means they can be called multiple times and don't mutate.

    //different functions taking in generic argumetns could specify
    //these traits in trait bounds (where statements)

    //functions can also implement these traits too


    //### Iterators ###

    //iterators are objects that allow you to iterate through the
    //elements of a sequence. in rust, they are lazy, i.e. they 
    //don't load data unless it is actually necessary. this prevents you 
    //from having to load the entire thing into memory at once

    use core::slice::Iter; //just to illustrate the complex typing

    let v: Vec<i32> = vec![1,2,3];
    let v_iter: Iter<'_, i32> = v.iter(); //first param is a lifetime

    //for x in y syntax automatically creates an iterator for y, 
    //but we can also use it on an iterator itself

    for val in v_iter {
        println!("val is {}", val);
    }

    //all iterators implement the next method, which takes in 
    //a mutable reference to the self and returns an Option that
    //either contains an item in the sequence or a None if the 
    //sequence is over

    //you can call next() in a naked way

    let mut v_iter = v.iter(); 
    //mut necessary because calling next() borrows as mutable
    //bc it changes the content 'loaded' into the iterator
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
    //you dont need to do this for a for loop bc the for loop takes
    //ownership of the iterator and makes it mutable behind the scenes

    //note: the iterator returns immutable references to items in the sequence
    //if we want to take ownership of the items, we can use into_iter()/
    //if we want to iterate over mutable references, we can do iter_mut

    //some iterators methods consume iterators. 
    //for example, the .sum() method defined on a Vec iterator
    //takes ownership of it, "consuming" it such that it can 
    //never be used again.

    //some methods create new iterators



    //recreate to set it back to beginning 
    let mut v_iter = v.iter();

    let plus_one_iter = v_iter.map(|x| x + 1); //iterator that gives each item +1

    for val in plus_one_iter {
        println!("val: {}", val);
    }

    //sometimes the closures you feed into an iterator 
    //method like this can capture values from their environments
    //in a useful way. like there is a .filter() method
    //that you call on an iterator that takes in a closure as a 
    //param. if the closure, evaluated on a single item, returns true,
    //filter gives the item. otherwise, it skips it. this is a common
    //way to collect all items fitting a specific criterion into one 
    //this closure would be a good one to capture a value in an environment,
    //say like a threshold, above which all items would be rejected

    //note: iterator and adapter abstractions are fast under the hood





    





















































}
