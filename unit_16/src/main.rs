use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use std::sync::Mutex;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // Fearless Concurrency

    //concurrent programming = where different parts of the code execute
    //independently

    //parallel progrramming = where different parts of the code execute
    //simultaneously

    //### Using Threads to Run Code Simultaneoulsy ###

    //Rust std lib uses a 1:1 model of creating threads, where
    //one thread specified in the language leads to one thread being 
    //executed by the OS

    //let's make a thread. we do it using thread::spawn() and pass
    //in a closure containing the code we want to run

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //assume the first line just read thread::spawn...
    //when the main thread completes, all spawned threads in the process
    //shut down, regardless of whether they have finished. so this code
    //will not print out all the numbers in the spawned thread up to 10

    //you cannot control the order in which threads are executed; the OS does

    //threads return an object of type JoinHandle. you can call .join() on it
    //to ensure that the thread finishes executing and doesn't get shut
    //down in the event that main() ends before it

    handle.join().unwrap();
    //anything below here will not happen until the spawned thread finishes
    //so if we put this line between the for loops, all the spawned thread
    //statements will finish, then all the main thread ones will

    
    //we often use the move keyword when making threads
    //a closure, which gets run in a thread, captures values from
    //its environment, often by borrowing them. -- note: it infers
    //what kind of ownership it needs to run the functionality in it.
    //it is conservative in that it takes the least strong level
    //of control it needs in order to do what it is supposed to do

    //this means it has 
    //references to all of them. but what if those objects could be
    //dropped while the thread is executing? it's often not possible
    //to know at compile time. so in these cases you have to add the 
    //move keyword to the closure in a thread to transfer ownership
    //of the values it captures to the thread. this ensures they will
    //not be dropped until the thread finishes executing

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("v is: {:?}", v);
    });//the closure is now the owner of v

    handle.join().unwrap();


    //### Message Passing ###

    /*

    it is kind of bad practice to have multiple threads that 
    share memory unless you abs have to. so what you do share information
    by message passing, a concept whereby threads send packets of 
    information back and forth to each other. it does this using 
    channels, which is a unidirectional connection between two actors
    the way it works is that a pair of objects called a transmitter and
    a receiver is created. calling a certain method on the transmitter
    passes something into the channel. calling 
    some method on the receiver accepts something out of the channel

    a channel is closed if either the transmitter or receiver is droppeed
    */


    //let's make a channel
    //mpsc = multiple producer, single consumer. 
    //channels can have many transmitters, but only one receiver

    let (tx, rx) = mpsc::channel();

    //let's make a thread and use it to send something into the transmitter
    // thread::spawn(move || { //move makes it so now the closure owns tx
    //     let vals: Vec<String> = vec![
    //         String::from("hi"),
    //         String::from("there"),
    //         String::from("from"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    //NOTE: putting this in comments bc otherwise it will take 
    //ownership of the transmitter and i want to use it later



    //ownership of a transmitter is necessary to use it
    //the send() method returns a result bc if the receiver has been 
    //dropped, then there is no place to send the information, so 
    //we get an error

    

    
    //the receiver has two methods: recv and try_recv
    //recv means receive, blocks the its thread's execution 
    //until a value is received from the transmitter
    //the try_recv tries to receive something, but if there isn't 
    //anything to receive, goes about doing other work. you could write
    //a loop that calls try_recv every so often

    //sending something down a channel transfers ownership of it
    //to the receiving party



    ////now let's receive something
    //let received = rx.recv().unwrap();
    //println!("Got: {}", received);

    // for received in rx {
    //     println!("received {}", received);
    // }//you can iterate over a receiver to do the same thing as recv().unpack()
    // //the main thread is waiting for each of the values received 
    // //during the iteration; it's not doing anything in between

    //NOTE: putting the above in comments bc want to reuse rx
    //below, and turning something into an iterator takes ownership of it


    //we can create multiple produers by cloning the transmitter

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("one"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
            String::from("two"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("received {}", received);
    }

    //note: originally tried to do let tx2 = tx.clone(); 
    //for the second thread and use that. that is BAD because
    //the way it works is that the iteration over rx never ends
    //until all transmitters have been dropped, which happens once
    //the threads they live inside have ended. but here i made
    //two clones and never used the orignal tx, so it wasn't dropped
    //it was waiting to be dropped at the end of the main function,
    //but the rx was waiting for it to be dropped before allowing
    //main to proceed. so this is a bug or something. lesson: 
    //don't have unused transmitters 


    // ### Shared State Concurrency ###

    //in addition to message passing, you can also do
    //concurrency using shared memory

    //you often use mutexes to accomplish this. mutex stands for 
    //mutual exclusion, and it's an object that makes it so that
    //only one thread can access a piece of memory at a time.
    //it has a thing called a lock that keeps track of which thread
    //currently has access.

    //when you use mutexes, your thread has to 1) signal that it
    //wants the lock before it manipulates the data 2) give up
    //the lock (or 'unlock') the data when it is done

    // a simple example

    //the way we access values inside a Mutex is by calling 
    //lock() on it. this requests the lock and makes it so that the current
    //thread has access to the data. the object returned is a LockResult,
    //which we unwrap(). inside that is a MutexGuard object, which is a 
    //type of smart pointer that points to the content inside the mutex

    //when the smart pointer goes out of scope, it automatically
    //gives up the lock to the mutex so that other threads can use it

    let m = Mutex::new(5);
    {
        //num (type MutexGuard) acts as a mutable reference to the data inside
        let mut num = m.lock().unwrap();
        *num = 6;//so we can dereference it to change the content of the mutex
    }//lock given up
    println!("num is {:?}", m);


    //sharing a mutex between multiple threads
    //let's make 10 threads and have them all increment a counter

    //note that if we just had a counter and had every thread 
    //modify it, there would be an error. the closures would need to 
    //take ownership of the of the counter, so after it would be 
    //moved to the first thread, it would be invalid for the second
    //thread to access it. this would be using a value after it was moved

    //one solution that comes to mind is Rc<T>, which allows there to 
    //be multiple owners of the same data. Unfortunately, that type isn't
    //implemented for use in multithreading bc it doesn't check to make sure
    //its way of keeping track of reference counts can't be messed up by 
    //threads stepping on each other's toes. so instead we use Arc<T>, a
    //type that IS meant for multithreading. it is an Atomic type,
    //which to means it is designed to be safe to share across threads

    //Atomics provide guarantees necessary for safe multithreading
    //at the cost of some performance stuff

    //Arc, or Atomic RC, has the same exact API as Rc

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        //make new smart pointer to the counter by cloning a borrowed
        //reference to it


        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);

    }

    //ensure that all threads terminate using join()
    for handle in handles {
        handle.join().unwrap();
    }

    println!("result is {}", *counter.lock().unwrap());

    //note: you could use this type of code structure to split
    //a computation up across threads and have them do their part and 
    //add up all the values at the end

    //if you want to do simple numeerical operations in a multi-threaded
    //way, there exist types for this simpler than Mutex<T> in 
    //std::sync::atomic for that

    //note: a Mutex itself is immutable but can give you a mutable
    //reference to the data inside it. this means it has interior 
    //mutability. this makes Mutex + Arc simiilar to RefCell + Rc


    //### Send and Sync Traits ###

    /// there are two traits that need to be implemented on types
    /// that make them possible to be used for multi-threading
    /// these are Send and Sync.
    /// 
    /// Send means a type can have its ownership transfeereed between threads
    /// most types in rust have Send trait so they can be transferred,
    /// but Rc<T> doesn't because putting diffrent references to the 
    /// same shared memory in multiple threads could mean the threads
    /// try to augment the reference count at the same time.
    /// Arc<T> solves this, but at a performance cost
    /// 
    /// nearly all primitives have the Send trait, and anything
    /// composed of all things with the Send trait has the Send trait
    /// 
    /// the Sync trait means multiple threads can access its memory
    /// aka a type is Sync if its reference &T is Send
    /// 
    /// since things that are composed of types that have the Send and Sync
    /// traits are themselves Send and Sync, you never really have to 
    /// implement these traits yourself. if you do, you need to use 
    /// unsafe Rust code. this is an expert level thing to do. 
    /// 
    /// 
























}
