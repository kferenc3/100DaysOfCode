use std::sync::mpsc; //multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); //channel returns a tuple
    let tx1 = tx.clone(); //multiple producers by cloning the transmitter

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),];
        for val in vals {
            tx1.send(val).unwrap(); //with move we move the tx into this thread. Then the send method returns a Result<T, E>
            thread::sleep(Duration::from_secs(1));
        }    
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("in"),
            String::from("thread 2!"),];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }    
        
    });

    //let received = rx.recv().unwrap(); //the receiver has recv and try_recv methods recv will block the main thread's execution and wait
                                       //for the transmitter. try_recv will not block and instantly return an Result<T, E>
    
    for received in rx {
        println!("Got: {}", received);
    }
}
