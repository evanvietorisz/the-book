use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs; //filesystem, used to read hello.html to string for response
use hello::ThreadPool;

//note: std::io::prelude is not the same as the Rust prelude, and 
//we need it bc it contains the Traits that allow us to call some 
//methods on the buffer


//Note: when testing this, open a browser and input the IP address below
//safari works better than chrome because chrome sends weird empty http 
//requests as part of some cacheing/preemptive content loading feature 
//that befuddle the logic in handle_connection(). 

fn main() {
    // Building a Multithreaded Web Server

    //first we need to listen for tcp requests
    //tcp is the information system upon which the syntax of http is built
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Error occurred creating TcpListener");
    //all IP addresses starting with 127 loop back to local server and
    //don't go through the internet. 7878 is a port that doesnt 
    //accept incoming HTTP from the internet so it is a good candidate
    //for this


    let pool = ThreadPool::new(4);

    ///a stream is a full cycle of a client connecting to server, 
    ///making a request, server generating a response, sending it back,
    ///.and closing the connection. we are writing the server side, 
    /// so the way this will work is that we will open a stream (from
    /// the client), write our response to it, and send it back.
    /// this loop represents all incoming streams for us to handle
    /// 
    /// note: incoming() doesn't just present us with valid connections; 
    /// it presents us with connection attempts, which could be invalid
    /// or coudld exceed the number of allowed open connections on the server
    
    //note: add .take(n) to limit the server to n requests before it does graceful shutdown
    for stream in listener.incoming() {//streams are type TcpStream
        let stream = stream.expect("Error occurred parsing stream"); 
        pool.execute(|| {
            handle_connection(stream);
        })

        // //old
        // let stream = stream.unwrap(); 
        // ///if you do nothing to handle the stream and run this code,
        // /// then access the IP address and port from your browser, you will
        // /// see the connection established print multiple times, 
        // /// indicating that a browser tried to connect. the reason multiple
        // /// connection messages print is that the browser often requests
        // /// multiple things, like page content, the little icon that goes
        // /// in the tab, etc. it may also just try multiple times if the connection fails
        
        // println!("Connection established!");
        // handle_connection(stream);
    }
    

}



///let's make a function that ingests the incoming stream
///and prints its contents. the contents is raw info so we read it
/// using a BufReader from std::io, an iterator over raw text like
/// file.open() in python
fn handle_connection(mut stream: TcpStream) {
    //NOTE: it is BAD to use two buffers on the same source (TcpStream) inside the same process
    //because under the hood they both share content loaded into memory and the the fact that 
    //they both read from it causes undefined behavior

    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    
    let http_request: Vec<_> = buf_reader //vector where we don't know what type will be returned from the iterator so we make it match anything
        .lines() //returns an iterator over lines of reader, splits at newline
        .map(|result| result.unwrap()) //each is originally a result<String, Err>
        .take_while(|line| !line.is_empty()) //says "spit out entries while they're nonempty"; end of message is denoted by two newline chars in a row
        .collect(); //and put them all into a vector
    println!("Request: {:#?}", http_request); //{:#?} is 'pretty printing', which uses the fmt::Display trait instead of fmt::Debug as in {:?}
    
    //only return a 200 if user requested the / page, else an error
    let request_line = http_request[0].clone(); //buf_reader.lines().next().expect("Error occurred using the next() method to get a line from buf_reader").expect("buf_reader contained an Error, not a String");
    

    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {//note: request_line is a String, which is automatically dereferenced to compare
        ( "HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    //note: we use this syntax because if we define the vars in two 
    //separate branches, they will go out of scope 
    //cannot define the existence of a variable conditionally
    
    let contents = fs::read_to_string(file_name).expect("Error occurred reading html"); 
    let length = contents.len(); 
    //the reason for including length is just related to html response protocol 
    
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    //write response back to stream
    stream.write_all(response.as_bytes()).expect("Error occurred writing response to stream");
    //automatically sends content back down the connection 

    println!("made it to end of handle_connection");
}


/*

how is this working?

we create a tcp connection and receive http requests through it

doing a for loop over each stream enacts an infinite loop that 
receives requests


for each incoming stream, we call threadpool.execute() and pass it an 
instruction (a Job) to handle it 

the threadpool works like this

when it is initialized, it creates a channel and a set of workers, which 
are wrappers on threads. the channel contains one input and originally one 
output, and it is a way by which threadpool sends the incoming jobs to
workers. upon initialization, the workers spawn a thread that runs an infinite
loop that constantly checks the output of the channel for a new task and executes it
to make this safe for multithreading, we have to wrap the output of the channel
(a receiver) in both a mutex (for thread safety) and an Arc, which is a 
thread-safe reference counted pointer that is rust's way of allowing multiple 
ownership

this code manifests as there being a queue down which jobs go, and when a 
thread finishs executing whatever it was working on, it checks the end of the 
channel, grabs the new job, and executes it

the way this works comes down to the fact that the workers each spawn an infinite
loop that constantly checks the end of the channel and executes the job there. 
if there is no job, the thread just continually checks for it, waiting without 
advancing. in the meantime, the other threads are blocked from accessing the end 
of the channel by the mutex, but that's okay because there are no jobs to do 
anyway. 

the specific behavior of the channel implementing a queue and the threads
running an infinite loop that just waits for a job to come through is what 
makes this process feel "online" and execute tasks as they come along


at shutdown, we do a couple of things

we dont want to cut off threads while they are doing a job, so 
we implement the Drop trait on ThreadPool such that it .join()s all the 
workers. this requires taking ownership of them, which is why we wrap them
in an Option so that we can call .take() on them. this would not be possible
if they weren't wrapped in an option; we would only be able to get a mutable
reference at best. 

if we .join() the workers to the main thread, then the workers need to 
actually terminate. that's why first drop the sender to delete the channel
and write logic in the workers to break out of their infinite loops if they
cannot connect to the channel, i.e. recv() returns an Error. this requires 
that we also wrap the sender in an Option so that we can take ownership of it 
to drop it. 






*/