use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

type Payload = i32;
type Message = (Payload, Sender<Payload>);

fn main() {
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Create a new thread
    let handle = thread::Builder::new()
        .name("actor".into())
        .spawn(move || {
            let (tx1, rx1): (Sender<Payload>, Receiver<Payload>) = mpsc::channel();
            tx.send((42, tx1)).unwrap();
            let num = rx1.recv().unwrap();
            println!("Got: {:?}", num);
        })
        .unwrap();

    let (num, tx1) = rx.recv().unwrap(); 
    tx1.send(num+1).unwrap();
    handle.join().unwrap();
}
