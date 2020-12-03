extern crate rand;
use rand::Rng;

use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};
use std::collections::VecDeque;

fn rand_sleep() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 10)));
}

// Need to make it cloneable so we can share it with the threads
#[derive(Clone)]
pub struct SemaPlusPlus<T> {
    queue_and_cv: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> SemaPlusPlus<T> {
    pub fn new() -> Self {
        SemaPlusPlus {queue_and_cv: Arc::new((Mutex::new(VecDeque::new()), Condvar::new()))}
    }

    // Enqueues -- Like semaphore.signal()
    pub fn send(&self, message: T) {
        let (queue_lock, cv) = &*self.queue_and_cv;
        let mut queue = queue_lock.lock().unwrap();
        let queue_was_empty = queue.is_empty();
        queue.push_back(message);
        if queue_was_empty {
            cv.notify_all();
        }
    }
    
    // Dequeues -- Like semaphore.wait()
    pub fn recv(&self) -> T {
        let (queue_lock, cv) = &*self.queue_and_cv;
        // Wait until there is something to dequeue
        let mut queue = cv.wait_while(queue_lock.lock().unwrap(), |queue| {
            queue.is_empty()
        }).unwrap();
        // Should return Some(...) because we waited
        queue.pop_front().unwrap()
    }
}

// Need to make it cloneable so we can share it with the threads
// impl<T> Clone for SemaPlusPlus<T> {
//     fn clone(&self) -> Self {
//         // TODO: implement!
//     }
// }

const NUM_THREADS: usize = 12;
fn main() {
    // Inspired by this example https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html
    let sem: SemaPlusPlus<String> = SemaPlusPlus::new();
    let mut handles = Vec::new();
    for i in 0..NUM_THREADS {
        let sem_clone = sem.clone();
        let handle = thread::spawn(move || {
            rand_sleep();
            sem_clone.send(format!("thread {} just finished!", i))
        });
        handles.push(handle);
    }
    for _ in 0..NUM_THREADS {
        println!("{}", sem.recv())
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
