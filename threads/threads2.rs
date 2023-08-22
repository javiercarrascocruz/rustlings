// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            match status_shared.lock() {
                Ok(mut i) => {
                    i.jobs_completed += 1;
                }
                _ => {}
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        match status.lock() {
            Ok(i) => {
                println!("jobs completed {}", i.jobs_completed);
            }
            _ => {}
        }
    }
}
