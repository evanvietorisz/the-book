fn main() {
    /*

    ### Ownership ####


    Stack and heap

    stack is where function calls go. 
    verything is fixed-size. sizes must be known
    LIFO

    heap 
    less organized, just free space to use
    things with unknown size at compile time or that change size need to go
    on heap
    when you allocate, an allocator finds a space of the right size,
    returns a pointer to it

    allocating memory on the stack is always faster than on the heap
    because the location to allocate the memory is always known
    on heap, the allocator has to search for a big enough place to 
    place the object

    accessing things on the heap is also slow bc you have to follow
    a pointer to get there

    when you call a function, the values you pass into that function 
    (potentially pointers) get pushed onto that function's 
    stack frame

    the main purpose of ownership is to manage data on the heap


    ### Ownership rules ###

    Three rules:
    - each value has an owner
    - there can only be one owner at a time
    - when the owner goes out of scope, the value is dropped

    whereas in C++ whenever you allocate heap memory, you 
    have to free it, in Rust, things are allocated manually, 
    but are freed automatically when they go out of scope
        by calling the drop() function on the object

    */

    //Let's look at an example with a String
    //note: a string literal is different from a String type
    //a sting literal is immutable, so it can just go on stack
    //a String is for variable length strings

    {
        let s: String = String::from("hello"); //space allocated by constructor String::from()
        println!("s is {s}");
    } // s automatically freed when it goes out of scope
      //calls drop() automatically

    //consider an example
    let s1 = String::from("hell0");
    let s2 = s1; //copies the pointer from s1 into s2
    //both vars point to the same place in mem
    //if we tried to free both, it would free the same 
    //place in mem two times, which can corrupt memory and be bad

    //so the way Rust handles this is that after letting s2
    //point to the memory allocated, s1 CEACES TO WORK!!

    //println!("s1 is {s1}"); does NOT WORK--gives "value borrowed after reference"

    //you need to clone the value at assignment to s2
    //let s2 = s1.clone();
    //println!("s1 is {s1}");


    //when we assign s1 to s2 above, it is not a full (deep) copy like clone()
    //nor is it a shallow copy, insofar as s2 becomes the SOLE 
    //thing that points to the relevant heap memory. so it's more than
    //just shallow copying the pointer info. it's called a MOVE 

    //this implies Rust never makes deep (costly) copies by default


    //in contrast, there are no issues with doing this stuff for 
    //fixed-size vars because they are allocated on the stack only

    let x = 5;
    let y = x; //ALL data of x (like its actual value) is copied to y

    println!("x is {x}"); //valid, no need to call x.clone()
    // since the data x represents is what its actual value is, 
    //there's no difference between deep and shallow copying here
    //all the data just gets copied
    //a formal way of describing this is that certain stack-native types
    //implement the Copy() method, which means they are copied in full
    //when their value is bound to another variable
    //these types do not and cannot implement the Drop method
    //so in all there are 2 kinds of types: ones that 
    //always live on stack and have Copy, and ones that live on heap
    //and have Drop

    /*
    Ownership and Functions

    passing things into functions is similar to assigning them to other vars

    if we have a String and pass it into a function, 
    then the string has now been moved to that function, and cannot
    be accessed again. it's as though we moved it to another variable.

    in contrast, if we take something like an int and pass it into 
    a function, since an int implements the Copy trait, it is passed
    entirely by value and the original variable can still 
    be used in the main script

    what would the lifetime of that string be? we'd declare it, 
    then pass it into a function, losing ownership of it. inside 
    the function, it would come into scope, be used in some way, 
    then be dropped at the end of that function execution

    UNLESS the function returned that string again, at which
    point it would come back into scope in the main script

    */

    let s: String = String::from("hello");

    let a = takes_and_gives_back(s); // so s is no longer a valid name, but the content is accessible thorugh a
    //when a goes out of scope, drop is finally called

    /*

    References

    references are a workaround to the idea of losing ownership over a
    variable. they are like pointers, except that they do not represent
    ownership of data. they simply point to that data without owning it.
    they are guaranteed to point to a certain type for the entirety of 
    their lifetime. 

    for example, if you had a string and wanted to do something with it 
    in a function without losing ownership of it, you;'d have to 
    pass it in as a reference

    note: a string IS a pointer to data on the heap, and making 
    a reference (pointer) that points to that pointer is 
    called BORROWING


    */

    let s: String = String::from("hello");
    let l = get_string_length(&s);
    println!("the length of {s} is {l}");
    //if we passed s itself into the function, 
    //we s would lose ownership of it, and it would not exist
    //for the println! statement

    /*

    we can have unlimited immutable references to something, 
    but if we have a MUTABLE reference, there can ONLY BE ONE, 
    and there cannot be other immutable references to that thing

    this prevents race conditions


    Dangling references

    say you had a function in which you made a string, then 
    returned a reference to it. 
    when the function would end, the string would be dropped
    but that would be bad, because the reference you would 
    be returning would then point to that deallocated memory

    this is called a dangling reference, and the compiler would 
    prevent you from doing this by saying "cannot return a borrowed
    value when there is nothing for it to be borrowed from"
    you could extend it with a lifetime if you needed to



    Slices

    slices are references to chunks of a sequence
    for example, a string


    */

    let s: String = String::from("hello world");
    let first_word = get_first_word(&s);
    println!("{first_word}");


    //string literals are immutable string slices (references)
    //they show up literally in the binary

    //you an also make slices of arrays

    let a: [i32; 5] = [1,2,3,4,5];
    let slice: &[i32] = &a[1..3]; //type is a reference to an array of i32s
    assert_eq!(slice, &[2,3]); 

    let b: bool = slice == &[2,3];
    println!("bool is {b}")

}

fn takes_and_gives_back(s: String) -> String {
    s
}//just takes it and gives it back

fn get_string_length(s: &String) -> usize {
    s.len()
}

fn get_first_word(s: &String) -> &str { // type symbol for string slice
    //note: it would be slightly better practice 
    //to make this take in a &str type so it could be run 
    //on both Strings and slices. for the former, 
    //you would just make a slice of the input String
    //this relates to smth called defref conversions


    // search for first space and return slice up to it. 
    //else return entire string
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; //slice indexing is like python
        }
    }
    &s[..] // slice to entire string; slices are references so return reference
}






