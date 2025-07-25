use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // Wrap JobStatus in a Mutex to allow safe, shared mutability.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the mutex before modifying the shared state.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Lock again to read safely.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
