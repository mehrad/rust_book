use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pool size must be greater than 0")
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type ArcJob = Arc<Mutex<mpsc::Receiver<Job>>>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));


        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<ArcJob>,
}

impl Worker {
    fn new(id: usize, receiver: ArcJob) -> Worker {
        let lock_error = "Failed to lock receiver mutex";
        let recv_error = "Failed to receive job from channel";
        let thread = thread::spawn(
            move || loop { //while let (and if let and match) does not drop temporary values until the end of the associated block
                let job = receiver.lock().expect(lock_error).recv().expect(recv_error);

                println!("Worker {} got a job; executing.", id);

                job();
            }
        );

        Worker { id, thread }
    }
}