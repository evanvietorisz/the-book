use std::collections::HashMap;

fn main() {
    /*

    Collections



    collections hold multiple elements and change size over time
    they are allocatedon the heap, unlike tuples and arrays

    the std::collections module contains all the collections

    it contains
    sequences: vector, vecdeque, and linked list
    maps: hashmap, btreemap
    sets: hashset, btreeset 
    misc: binaryhheap


    */

    //### Vector ###
    
    // Vec<T>

    //declare empty vec
    let v: Vec<i32> = Vec::new(); //specify type of entries in <>
                                    //bc we didn't pass anything in to infer from
    //note: you dont have to specify the type
    //if you don't,it will be specified at first push to v


    //declare vec from elements using vec! macro
    let v = vec![1,2,3,4]; //defaults to i32 bc thats default type for ints

    //update a vector with .push
    //requires vec to be mutable 
    let mut v = Vec::new();
    v.push(5); //infers that v is type Vec<i32>
    v.push(6);
    v.push(7);

    //accessing elements in a vector

    //you can access elements in a vector by indexing or using the get method

    //access via indexing, gives a REFERENCE to the value
    let third: &i32 = &v[2];

    //this method also gives you a reference
    let third: Option<&i32> = v.get(2); //option that could contain a reference to i32
    match third {
        Some(third) => println!("third element is {}", third),
        None => println!("there is no third element"),
    }

    //indexing past the length of the the vec causes a panic error and program crashes
    //using .get() on an index past the length of the array returns an Option<T>,
    //which you can then handle

    //borrowing in the context of grabbing vec elements
    /*
    recall the rule that states that at any time you can have either
    unlimited immutable references or one mutable reference existing
    at any time. that comes into play when you are holding references
    to specific elements in a vector and try to update it

    the .push() method under the hood takes in the parent object 
    instance via a MUTABLE reference,
    so you cannot create an immutable reference to a specific element of
    a vec, then push something to it using .push() because that would mean 
    there exist an immutable and a mutable reference to the vec at the same
    time. on an intuitive level, the reason this would not be allowed
    is that the .push() could force the vec (dynamic array) to be copied
    to another location in memory, so having an immutable reference 
    to one of its elements could end up pointing to deallocated memory 
    after the .push(), which would be a seg fault

    */

    //iterating over values in a vector

    //using for i in &v or for i in &mut v iterates over the sequence of 
    //immutable or mutable references to its elements
    let mut v: Vec<i32> = vec![1,2,3,4,5];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 5; //need to use the DEREFERENCE operator to manipulate
                 //the value pointed to by a mutable reference
        println!("{}", i);
    }

    //note: we don't run into issues with the borrow checker because
    //after the for loop finishes running, the immutable references
    //don't exist anymore, so we can easily create mutable ones
    //in the second for loop

    //vectors can only hold objects of the same type. 
    //this is annoying, so you can get around it by using enums,
    //since every variant of an enum is the same type

    //if you don't know all the variants up front, enums won't work,
    //and you have to use trait objects 

    //a vector implements all kinds of other useful methods like pop

    //like any other struct, when a vector is dropped, all its elements
    //are dropped

    // ### String ###

    /*

    core rust has the str type, which is binary interpreted 
    as a string. &str is the common form of it, and is the type of a 
    string slice. string literals are actually string slices of this type 
    because the text itself is stored in the program's binary, and we use
    &str references to refer to them.

    The String type is in the collections library and is a mutable,
    growable sequence. 

    */

    //string literals are same type as string slices
    let data: &str = "string literal";

    //create a String out of it
    let s: String = data.to_string(); 
    let s: String = String::from("equivalent to the above");
    //note: all types that have the 'display' trait have the 
    //to_string() method

    //we can initialize an empty string by 
    let mut s: String = String::new();

    //anything that is utf-8 encoded can be a string

    //it's not a coincidence that Strings have many of the same
    //methods as Vecs because Strings are actually a wrapper on 
    //the Vec<T> type

    //appending to a string
    let mut s: String = String::from("foo");
    //push_str() appends a string slice &str without taking ownership
    let s2: &str = "bar";
    s.push_str(s2);
    println!("{}", s);
    //push() pushes a single char 
    s.push('p');
    println!("{}", s);

    //those methods add a &str to a String and a char to a String

    //combining two Strings

    // + operator
    let s1 = String::from("Hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2; 
    //this moves s1 to s3 so s1 is no longer a valid variable
    //the reason we move s1 but only take a reference i.e. borrow s2
    //is that this + is implementing a .add(self, &str) method on s1
    //the reason we can pass in &String to somethign that takes in 
    //&str is because of dereferencing coersion, a feature that coerces
    //the &string into a &str under the hood of the operator

    //note: there is no way to combine two Strings

    //you can also use the format! macro to combine strings
    //when the + operator would be too tedious

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); //note these are borrowed
    println!("{}", s);
    println!("{}", s1);

    /*
    you cannot index into strings in rust. the reason is that
    when you index, you are basically saying "go ahead x bytes
    from the start of the string and return what is there". that
    doesnt work in strings because characters in different languages
    are encoded using differnet numbers of bytes. for example, each letter
    in the russian alphabet is 2 bytes, whereas each letter in english
    is 1 byte. things get further complicated by the underlying representations
    of things like accent marks and other languages that combine letters

    since when you index using a single value into a string it is 
    ambiguous whether you want a byte (number), character, or collection 
    of characters analogous to a letter (like a whole collection of letters
    into a cell in korean). also, it's not possible to have O(1) indexing
    because of these ambiguities anyway. so indexing is not implemented

    you CAN do slicing, but if you try to do a slice that doesn't correspond
    to a valid list of characters, like try to slice 3 bytes of a string
    written in a language where each letter is 2 bytes long, it will crash
    your program

    */

    //rust implements methods on strings that allow you do to 
    //all the things you want to do without running into the 
    //issues of direct integer indexing or slicing

    //.chars() gives chars
    for c in "Зд".chars() {
        println!("{c}");
    }

    //.bytes() gives the numbers encoded by the bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    //getting clusters like you'd need to do in korean is more complex
    //and you need external crates to do it 

    // ### Hash maps ###
    
    //need to bring in manually
    //use std::collections::HashMap;

    //HashMap<K, V> must have all keys, values be of same type as each other

    //insert a value using .insert(key, value)

    let mut scores: HashMap<String, i32> = HashMap::new(); //no declared type of key, values
    scores.insert(String::from("Blue"), 3);
    scores.insert(String::from("Red"), 10);

    //access a value using .get

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //this feeds in a borrowed reference to the (heap allocated)
    //team_name, and the .get() method returns an option reference 
    //to the corresponding value Option<&i32>. .copied() converts
    //that to an Option<i32> to give you a copy of the value itself.
    //then unwrap_or(0) gives you the value of it's Some(value) and 0
    //if it's None, i.e. there was no entry associated with that key

    //iterate over hashmap
    for (key, value) in &scores { //do a reference to hashmap
        println!("{key}: {value}");
    }
    //things are printed in arbitrary order 

    //ownership in hashmaps
    //if you use objects of types that implement the copy trait
    //as keys or values, the variables that refer to them will be 
    //valid even after they are used as keys. for heap-allocated
    //types that don't, ownership gets transferred when the variable
    //is used as a key or value

    //we can insert references to values onto the hashmap if we don't 
    //want to lose ownership of them, but that requires us to specify 
    //that the lifetime of the reference is at least as long 
    //as the lifetime of the hashmap to ensure that the reference
    //is valid. this is a later topic

    //updating a hashmap

    //overriding previous value: insert()
    let mut my_map: HashMap<String, i32> = HashMap::new();

    my_map.insert(String::from("Blue"), 10);
    my_map.insert(String::from("Blue"), 20);

    for (key, value) in &my_map {
        println!("{key}: {value}");
    }

    //inserting a value if the corresponding key isn't in the hashmap
    my_map.entry(String::from("Blue")).or_insert(50);
    //"insert Blue: 50 if blue not already in HashMap"
    //entry gives an Option<&i32> to the value associated with Blue

    //manipulate the value associated with a particular key

    let text = "hello hello world";
    let mut map = HashMap::new(); 

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //count is a reference to the value
        //use dereference operator to actually manipulate the value itself
    }

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    //the std library hashmap uses the SipHash hash alg
    //others are available in crates.io

}
