use std::thread;
use std::time::Duration;

fn main() {
    // create_thread();
    // join_thread();
    closure_thread();
}

fn create_thread() {
    // Automatically cleaned up when the main thread ends, so 5-9 never printed
    thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..5 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(10));
    }
}

fn join_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..5 {
        println!("{} from main thread before join", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();

    for i in 1..5 {
        println!("{} from main thread after join", i);
        thread::sleep(Duration::from_millis(10));
    }
}

fn closure_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    // Compiler error, use after move
    // drop(v);

    handle.join().unwrap();
}
