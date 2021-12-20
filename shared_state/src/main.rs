use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Modify a Mutex by acquiring the lock
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // To share a Mutex across threads it must be wrapped in an Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count: {}", *counter.lock().unwrap());
}
