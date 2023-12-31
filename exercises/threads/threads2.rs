// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


//// This felt more painful than needed?
// `You must take an action before you update a shared value` 
// really suggested to me that the Arc setup was good
// I'm still not sure if this was the intent

// ya, arc doesn't allow mutability by default -> need to use atomics or mutex
//// ref: https://doc.rust-lang.org/std/sync/struct.Arc.html

use std::sync::Arc;
use std::thread;
use std::time::Duration;
//use std::sync::Mutex; // Option.1
use std::sync::atomic::{AtomicUsize, Ordering}; // Option.2

struct JobStatus {
    // Original
//    jobs_completed: u32,
    // Option.2
    jobs_completed: AtomicUsize,

}

fn main() {
    // Original
//    let status = Arc::new(JobStatus { jobs_completed: 0 });
    // Option.1  
//    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // Option.2
    let status = Arc::new(JobStatus { jobs_completed: AtomicUsize::new(0) });

    let mut handles = vec![];
    for _ in 0..10 {

        // Orginal & Option.2
        let status_shared = Arc::clone(&status);
        // Option.1
//        let status_shared = status.clone();


        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value
            // Original
//            status_shared.jobs_completed += 1;

            // Option.1
//            status_shared.lock().unwrap().jobs_completed += 1;

            // Option.2
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // Original
//        println!("jobs completed {}", ???);
        // Option.1
//        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        // Option.2
        println!("jobs completed {}", status.jobs_completed.load(Ordering::SeqCst));
    }
}
