use std::sync::mpsc; // MSG
                     // mpsc = multiple producer, single consumer
use std::sync::{Arc, Mutex}; // Mutex
use std::thread;
use std::time::Duration;

fn main() {
    println!("## Using Threads to run code simultaneously\n");

    // API ---
    threaddy();
    println!();
    handy();
    println!();
    dandy();
    println!();
    mandy();

    println!("## Using Message Passing to Transfer Data Between Threads\n");
    // MSG ---
    channel_one();
    println!();
    channel_deluge();
    println!();
    channel_factory();

    println!("## Shared-State Concurrency\n");
    // Mutex ---
    mutex_toot();
    println!();
    mutex_share();
}

// API ---
fn threaddy() {
    println!("---| Creating a New Thread with `spawn`");
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn handy() {
    println!("---| Waiting for All Threads to Finish Using `join` Handles");
    let handel = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handel.join().unwrap(); // music to my ears
}

fn dandy() {
    println!("---| Waiting for All Threads to Finish Using `join` Handles");
    let handel = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handel.join().unwrap(); // music to my ears
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn mandy() {
    println!("---| Using `move` Closures with Threads");

    let v = vec![1, 2, 4];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// MSG ---

fn channel_one() {
    println!("---| Sending a single value");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn channel_deluge() {
    println!("---| Sending Multiple Values and Seeing the Reveiver Waiting");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn channel_factory() {
    println!("---| Creating Multiple Producers by Cloning The Transmitter");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("(more)"),
            String::from("(messages)"),
            String::from("(for)"),
            String::from("(you)"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Mutex ---
fn mutex_toot() {
    println!("|--- The API of `Mutex<T>`");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // num goes out of scope here and the lock is free.

    println!("m = {:?}", m);
}

fn mutex_share() {
    println!("|--- Sharing a `Mutex<T>` Between Multiple Threads");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
