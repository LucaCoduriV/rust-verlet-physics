use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

type JobId = Box<dyn FnOnce(usize) + Clone + Send + 'static>;

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
            thread_pool: thread_pool,
            receiver,
        }
    }

    pub fn execute_on_all(&self, work: JobId) {
        for worker in self.thread_pool.iter() {
            worker.execute(Box::new(|id| {
                work();
            }));
        }
    }

    pub fn wait_all_finish(&self) {
        for _ in 0..self.thread_pool.len() {
            let worker_id = self.receiver.recv().unwrap();
            println!("{worker_id} finished !");
        }
    }
}

enum WorkerMessage{
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
            sender.send(0).unwrap();
        }
    }

    fn execute(&self, job: JobId)
    {
        self.sender.send(WorkerMessage::NewJob(job)).unwrap();
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
        })
    }
}