// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


// Observation.1.a => if send_tx is commented out, the main fn hangs due to `for received in rx`
//      inference: `tx` is the transmitter and `rx` is receiver & `rx` is waiting for msgs
// Observation.1.b => `let queue_length = queue.length;` has a size of 10 ...weird?  why / where did that come from?
//      inference: in my mind "queue" and "channel" are doing the same work, but here queue might be more like a range?

// Observation.2.a => if I bring back `send_tx` but comment a thread, code works fine (fails for 5 != 10 assertion)
//      inference: the error experienced when running the code raw is based on reusing the sender accross threads

// Yep - that seems to have done it, everything works now


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}


fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}


#[test]
fn main() {
    let (tx, rx) = mpsc::channel::<u32>();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }
    
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
