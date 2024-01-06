use std::thread;
use std::sync::{mpsc, Mutex, Arc};

//we want to have threads wait around for instructions to come around 
//for them to work on. but std only provides an implementation where 
//a thread executes code it is spawned with. we have to implement 
//waiting around ourselves. so we will create a Worker struct that 
//holds a thread initialized with empty code. that thread will then 
//wait around to receive further instructions
struct Worker {
    id: usize, 
    thread: Option<thread::JoinHandle<()>>, //option so that we can .take() ownership of it for graceful shutdown
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {//move gives the thread the ability to take ownership of the (clone of the) receiver
            loop {
                let message: Result<Job, mpsc::RecvError> = receiver.lock().expect("Thread {id} encountered error aquiring lock").recv(); //

                match message {
                    Ok(job) => {
                        //request lock (to enter mutex), then call the receive method on the enclosed receiver
                        //recv blocks things from happening until it receives something
                        //so if there is no job available in the receiver, the thread will just 
                        //retain the lock, waiting around for one to come by
                        println!("worker {id} got a job; executing.");

                        job();
                    },
                    Err(_) => {
                        println!("worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { 
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>; //type alias

pub struct ThreadPool {
    workers: Vec<Worker>, // JoinHandles "point" to threads, and the () denotes that the threads don't return anything
    sender: Option<mpsc::Sender<Job>>, //option so that we can .take() ownership of it for graceful shutdown
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics 
    /// 
    /// The `new` function will panic if the size is zero. 
    /// 
    /// If there are not enough system resources to create a new thread,
    /// this struct will panic! A more robust solution would be to use a
    /// `std::thread::Builder`` to spawn new threads, which returns a `Result`
    pub fn new(size: usize) -> ThreadPool { //note: new by convention panics, whereas build returns an Error
        //TODO: implement this as a build where it returns a custom PoolCreationError
        assert!(size > 0); //0 threads makes no sense; neg number is impossible bc of u32 type
        

        let (sender, receiver) = mpsc::channel(); //why no need to type annotate

        //in order to allow multiple workers to co-own the receiver in a thread-safe way, use a Mutex (prevents race conditions) and and Arc (reference counter)
        let receiver = Arc::new(Mutex::new(receiver)); 

        let mut workers = Vec::with_capacity(size); //store the num threads as a constraint on the vector, not an attribute
        //with_capacity preallocates the space

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { 
            workers, 
            sender: Some(sender),
        }
    }
    
    /// We want to do graceful shutdown. That means that when the web server
    /// shuts down for any reason, we want all threads that are currently executing to
    /// finish before shutting down as opposed to just halting them then and there. 
    /// We do this by implementing the Drop trait on ThreadPool to customize what
    /// happens when it is dropped. We first drop the sender, which closes the channel
    /// between the threadpool and its workers. (we implement logic in the workers that
    /// terminates their threads gracefully if the connection closes). Then, we join() all
    /// the threads of the workers to ensure that the main thread doesn't exit until 
    /// they all finish. 

    //looking at the documentation for thread::spawn, we need FnOnce and Send and 'static.
    //the latter two are bc we want to be able to transfer the closure between
    //threads and because if this closure can be passed from thread to thread, it 
    //could be passed to a new thread that outlives the one it was created in. it
    //needs to be valid all the time for this to be possible (as long as the main thread is active) 
    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static //need the () bc FnOnce goes FnOnce(inputs) -> outputs. we omit outputs, but still need to specify empty input field
    {
        let job = Box::new(f);

        self.sender.as_ref().expect("Sender was None!").send(job).expect("Error occurred when sender sent job through channel!");
        //as_ref necessary so .expect() doesn't take ownership of the sender field
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take()); //necessary so that the threads actually terminate, allowing main to terminate

        for worker in &mut self.workers {//&mut necessary bc we need to mutate the workers
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {//if worker's thread is a Some, .take() ownership of it and join it
                thread.join().expect("Could not join worker {id}'s thread at shutdown"); 
            }
        }
    }
}

/* 

Note: 

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}

would not produce the desired result. it would compile, but 
jobs would not be able to run simultaneoulsy. the reason is that
the mutex lock would not be released at the desired time. 

we want the thread to access the end of the channel, obtain a job,
and give up the lock so that other threads can access it. this isn't 
going to happen because renouncing the lock only happens when certain 
references created at the .lock() call are dropped. in normal let statements,
this happens when the assignment is finished, so there are no problems. 
but in while let, if let, and match, none of the references that are created on the 
RHS of the assignment are dropped until the entire while, if, match block 
is finished executing. so here, the mutex lock is not given up until 
the job is finished running, which is NOT what we want!

*/