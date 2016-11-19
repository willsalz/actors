// MSPC == 'multiple sender, single receiver'
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

type Payload = i32;
type Message = (Payload, Sender<Payload>);

fn main() {
    // Actor Mailbox
    let (outbox, inbox): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Create a new thread
    let handle = thread::Builder::new()
        .name("actor".into())
        .spawn(move || {
            // Forever...
            loop {
                // See if we have messages!
                match inbox.recv() {
                    // If we're given a 'special' -1 value, exit.
                    Ok((num, _)) if num == -1 => {
                        println!("[Actor] Exiting!");
                        break;
                    }
                    // Otherwise, print and respond to the message!
                    Ok((num, sender)) => {
                        println!("[Actor] Got: {:?}", num);
                        sender.send(num + 1).unwrap();
                    }
                    // Exit on error
                    Err(e) => {
                        println!("[Actor] Error: {:?}", e);
                        break;
                    },
                }
            }

        })
        .unwrap();

    // Communication channel with actor
    let (sender, receiver): (Sender<Payload>, Receiver<Payload>) = mpsc::channel();
    outbox.send((0, sender.clone())).unwrap();
    let num = receiver.recv().unwrap();
    println!("[Main] Got: {:?}", num);

    // Send another number!
    outbox.send((num + 1, sender.clone())).unwrap();
    let num = receiver.recv().unwrap();
    println!("[Main] Got: {:?}", num);

    // Send exit code
    outbox.send((-1, sender.clone())).unwrap();

    // Wait for thread to exit
    let res = handle.join().unwrap();
    println!("[Main] Res: {:?}", res);
}
