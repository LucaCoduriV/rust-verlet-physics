use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

type JobId = Box<dyn FnOnce(usize) + Send + 'static>;

pub struct WorkManager
{
    thread_pool: Vec<Worker>,
    receiver: Receiver<usize>,
}

impl WorkManager {
    pub fn new(number_worker: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut thread_pool = Vec::with_capacity(number_worker);
        for i in 0..number_worker {
            let worker = Worker::new(i, sender.clone());
            thread_pool.push(worker);
        }


        Self {
            thread_pool,
            receiver,
        }
    }

    pub fn execute_on_all<F>(&self, work: F) where
        F: Fn(usize) + Send + Sync + 'static
    {
        let work = Arc::new(work);
        for worker in self.thread_pool.iter() {
            let f = work.clone();
            worker.execute(move |id| {
                f(id);
            });
        }
    }

    pub fn wait_all_finish(&self) {
        for _ in 0..self.thread_pool.len() {
            let worker_id = self.receiver.recv().unwrap();
            println!("{worker_id} finished !");
        }
    }
}

impl Drop for WorkManager{
    fn drop(&mut self) {
        for _ in 0..self.thread_pool.len() {
            let worker = self.thread_pool.remove(self.thread_pool.len() - 1);
            worker.sender.send(WorkerMessage::Terminate).unwrap();
            worker.thread_handle.join().unwrap();
        }
    }
}

enum WorkerMessage {
    NewJob(JobId),
    Terminate,
}

struct Worker {
    thread_handle: JoinHandle<()>,
    sender: Sender<WorkerMessage>,

}

impl Worker {
    fn new(worker_id: usize, state_sender: Sender<usize>) -> Self {
        let (job_sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || Self::job_loop(receiver, worker_id, state_sender));
        Self {
            thread_handle: handle,
            sender: job_sender,
        }
    }

    fn job_loop(recv: Receiver<WorkerMessage>, worker_id: usize, sender: Sender<usize>) {
        loop {
            let message = recv.recv().unwrap();
            match message {
                WorkerMessage::NewJob(job) => {
                    job(worker_id);
                }
                WorkerMessage::Terminate => {
                    break;
                }
            }
            sender.send(worker_id).unwrap();
        }
    }

    fn execute<F>(&self, job: F) where
        F: FnOnce(usize) + Send + 'static
    {
        self.sender.send(WorkerMessage::NewJob(Box::new(job))).unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::WorkManager;

    #[test]
    fn execute_println_4_thread() {
        const NB_THREAD: usize = 4;
        let wm = WorkManager::new(NB_THREAD);
        wm.execute_on_all(|id| {
            println!("Thread: {}", id);
        });

        wm.wait_all_finish();
    }
}