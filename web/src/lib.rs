use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    // Factory method.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        // Arc == Atomic Reference Counter shared ref.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        // generic is not the same as `Job`
        // the FnOnce is trait, in order to own it
        // we need to wrap in <dyn FnOnce> to present as a trait object.
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("start dropping the thread pool, as well as all worker thread within");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("TERM signal all send out");
        for worker in &mut self.workers {
            println! {"Shutting douwn worker no. {}", worker.id};
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Factory Method.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // `move` similar to C++ closure
        let _thread = thread::spawn(move || loop {
            // Receiver lock will drop before executing the job.
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job ", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} will terminate", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(_thread),
        }
    }
}
