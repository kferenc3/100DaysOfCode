use std::thread;
use std::time::Duration;


fn main() {

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move|| { //move ownership from main thread to spawned
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            println!("Here is the vector moved here from the main thread: {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } //in case the main thread finishes that will also terminate the spawned thread.

    handle.join().unwrap(); //with the JoinHandle the execution will wait for the spawned thread to finish.
}
