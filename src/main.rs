#![feature(unboxed_closures, fn_traits, rustc_private)]

#[macro_use]
extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

type Payload = i32;

struct Message {
    payload: Payload,
    sender: Sender<Payload>,
}

trait Actor {
    fn handle(&self, message: Message) -> ();
}

struct ActorHarness<T: Actor> {
    inbox: Receiver<Message>,
    actor: T,
}

impl<T: Actor> ActorHarness<T> {
    fn new(inbox: Receiver<Message>, actor: T) -> ActorHarness<T> {
        ActorHarness {
            inbox: inbox,
            actor: actor,
        }
    }
}

impl<T: Actor> FnOnce<()> for ActorHarness<T> {
    type Output = ();
    extern "rust-call" fn call_once(self, _: ()) -> () {
        // Forever...
        loop {
            // See if we have messages!
            match self.inbox.recv() {
                Ok(m) => self.actor.handle(m),
                // Exit on error
                Err(e) => {
                    error!("[Actor] Error: {:?}", e);
                    break;
                }
            }
        }
    }
}

struct ActorRef {
    outbox: Sender<Message>,
}

impl ActorRef {
    fn new(outbox: Sender<Message>) -> ActorRef {
        ActorRef { outbox: outbox }
    }
}

struct ActorImpl {
}

impl ActorImpl {
    fn new() -> ActorImpl {
        ActorImpl {}
    }
}

impl Actor for ActorImpl {
    fn handle(&self, message: Message) -> () {
        match message {
            // If we're given a 'special' -1 value, exit.
            Message { payload, sender: _ } if payload == -1 => {
                info!("[Actor] Exiting!");
                panic!("Get me out of here!");
            }
            // Otherwise, print and respond to the message!
            Message { payload, sender } => {
                info!("[Actor] Got: {:?}", payload);
                sender.send(payload + 1).unwrap();
            }
        }
    }
}

fn main() {
    // Actor Mailbox
    let (outbox, inbox): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // Meet the players
    let actor = ActorImpl::new();
    let harness = ActorHarness::new(inbox, actor);
    let r = ActorRef::new(outbox);

    // Create a new thread
    let handle = thread::Builder::new()
        .name("actor".into())
        .spawn(harness)
        .unwrap();

    // Communication channel with actor
    let (sender, receiver): (Sender<Payload>, Receiver<Payload>) = mpsc::channel();
    r.outbox
        .send(Message {
            payload: 0,
            sender: sender.clone(),
        })
        .unwrap();
    let num = receiver.recv().unwrap();
    info!("[Main] Got: {:?}", num);

    // Send another number!
    r.outbox
        .send(Message {
            payload: num + 1,
            sender: sender.clone(),
        })
        .unwrap();
    let num = receiver.recv().unwrap();
    info!("[Main] Got: {:?}", num);

    // Send exit code
    r.outbox
        .send(Message {
            payload: -1,
            sender: sender.clone(),
        })
        .unwrap();

    // Wait for thread to exit
    let res = handle.join();
    info!("[Main] Res: {:?}", res);
}
