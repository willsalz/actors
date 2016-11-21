#![feature(unboxed_closures, fn_traits)]
// MSPC == 'multiple sender, single receiver'
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

type Payload = i32;

struct Message {
    payload: Payload,
    sender: Sender<Payload>,
}

struct Actor {
    inbox: Receiver<Message>
}

impl Actor {
    fn new(inbox: Receiver<Message>) -> Actor {
        Actor{inbox: inbox}
    }
}

impl FnOnce<()> for Actor {
    type Output = ();
    extern "rust-call" fn call_once(self, _: ()) -> () {
        // Forever...
        loop {
            // See if we have messages!
            match self.inbox.recv() {
                // If we're given a 'special' -1 value, exit.
                Ok(Message{payload, sender: _}) if payload == -1 => {
                    println!("[Actor] Exiting!");
                    break;
                },
                // Otherwise, print and respond to the message!
                Ok(Message{payload, sender}) => {
                    println!("[Actor] Got: {:?}", payload);
                    sender.send(payload + 1).unwrap();
                },
                // Exit on error
                Err(e) => {
                    println!("[Actor] Error: {:?}", e);
                    break;
                },
            }
        }
    }
}

struct ActorRef {
    outbox: Sender<Message>
}

impl ActorRef {
    fn new(outbox: Sender<Message>) -> ActorRef {
        ActorRef{outbox: outbox}
    }
}

fn main() {
    // Actor Mailbox
    let (outbox, inbox): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let actor = Actor::new(inbox);
    let r = ActorRef::new(outbox);

    // Create a new thread
    let handle = thread::Builder::new()
        .name("actor".into())
        .spawn(actor)
        .unwrap();

    // Communication channel with actor
    let (sender, receiver): (Sender<Payload>, Receiver<Payload>) = mpsc::channel();
    r.outbox.send(Message{payload: 0, sender: sender.clone()}).unwrap();
    let num = receiver.recv().unwrap();
    println!("[Main] Got: {:?}", num);

    // Send another number!
    r.outbox.send(Message{payload: num + 1, sender: sender.clone()}).unwrap();
    let num = receiver.recv().unwrap();
    println!("[Main] Got: {:?}", num);

    // Send exit code
    r.outbox.send(Message{payload: -1, sender: sender.clone()}).unwrap();

    // Wait for thread to exit
    let res = handle.join().unwrap();
    println!("[Main] Res: {:?}", res);
}
