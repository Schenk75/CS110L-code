use std::sync::{Arc, Mutex};

use std::{thread, time};
use rand::Rng;

fn handle_call() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 10)));
}

fn take_break() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0, 10)));
}

fn should_take_break() -> bool {
    rand::random()
}

fn ticket_agent(id: usize, remain: Arc<Mutex<usize>>) {
    loop {
        let mut remain_ref = remain.lock().unwrap();
        if *remain_ref == 0 {
            break;
        }
        handle_call();
        *remain_ref -= 1;
        println!("Agent #{} sold a ticket! ({} more to be sold)", id, *remain_ref);
        if should_take_break() {
            take_break();
        }
    }
    println!("Agent #{} notices all tickets are sold, and goes home!", id);
}

fn main() {
    let remain = Arc::new(Mutex::new(250));

    let mut threads = Vec::new();
    for i in 0..10 {
        let remain_ref = remain.clone();
        threads.push(thread::spawn(move || {
            ticket_agent(i, remain_ref);
        }));
    }

    for handle in threads {
        handle.join().expect("Panic occurred in thread!");
    }

    println!("End of business day!");
}
