mod fnbox;
mod worker;

use worker::{Worker, Message};
use fnbox::FnBox;
use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> Self {
        let (sx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let rec: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(rx));
        let mut personal_workers: Vec<Worker> = Vec::with_capacity(pool_size);

        for worker_id in 0..pool_size {
            personal_workers.push(Worker::new(worker_id, Arc::clone(&rec)));
        }

        ThreadPool { workers: personal_workers, sender: sx }
    }

    pub fn execute<F: FnBox + Send + 'static>(&self, func: F) {
        let task: Box<dyn FnBox + Send + 'static> = Box::new(func);
        self.sender.send(Message::NewTask(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down server...");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {

            if let Ok(w) = worker.get_worker_ownership() {
                w.join().unwrap();
            }

        }

    }
}