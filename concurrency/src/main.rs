use std::sync::{Arc, Mutex}; //mutual exclusion allows only one thread to access data at any given time. Arc is 'atomic' RC<T> atomic means it is safe to share between threads
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(1)); //creating an Arc Mutex makes sure the we can pass around the value between threads safely
    let mut handles = vec![];

    for _ in 1..10 { //spawning 10 threads
        let counter = Arc::clone(&counter); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); //first a lock needs to be acquired. The lock will block the thread until it can acquire the lock, or panic another thread holding the lock panics

            *num += 1; //each threads will add 1 to the counter and num will go out of scope
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap(); //the join makes sure the main thread doesn't finish before the threads
    }

    println!("Result: {}", *counter.lock().unwrap()); //main locks the mutex at the end and prints the value inside it.
}
