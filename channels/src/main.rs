use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Sending a single value

    // mpsc stands for "multiple producers single consumer"
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // Compiler error, `tx.send` moved `val`
        // println!("{}", val);
    });

    // Sending multiple values

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // Multiple producers

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("a1"),
            String::from("b1"),
            String::from("c1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("a2"),
            String::from("b2"),
            String::from("c2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
