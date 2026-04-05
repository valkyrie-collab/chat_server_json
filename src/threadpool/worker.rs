use crate::threadpool;
use std::thread::{ self, JoinHandle };
use std::sync::{ Arc, Mutex, MutexGuard, mpsc::Receiver };
use threadpool::fnbox::Task;
use std::io::Error;

pub enum Message {
    NewTask(Task),
    Terminate    
}

pub struct Worker {
    id: usize,
    worker: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(worker_id: usize, rec: Arc<Mutex<Receiver<Message>>>) -> Self {
        let personal_worker: JoinHandle<()> = thread::spawn(move || {
            
            loop {
                let msg: Message = {
                    let rx: MutexGuard<Receiver<Message>> = rec.lock().unwrap();
                    rx.recv().unwrap()
                };
                
                match msg { 
                    Message::NewTask(t) => {
                        println!("Doing task with id: {}", worker_id);
                        t.call_box();
                    }
                    Message::Terminate => {
                        println!("Terminating task with id: {}", worker_id);
                        break;
                    }
                }
                
            }
            
        });
        
        Worker { id: worker_id, worker: Some(personal_worker) }
    }
    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_worker_ownership(&mut self) -> Result<JoinHandle<()>, Error> {

        if let Some(task) = self.worker.take() {
            Ok(task)
        } else { panic!("There is no thread") }

    }
}