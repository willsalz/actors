#![feature(unboxed_closures, fn_traits, rustc_private)]

#[macro_use]
extern crate log;
extern crate env_logger;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// Lib
#[derive(Debug)]
struct Envelope<In, Out> {
    input: In,
    output: Sender<Out>,
}

impl<In, Out> Envelope<In, Out> {
    fn new(input: In, output: Sender<Out>) -> Self {
        Envelope {
            input: input,
            output: output,
        }
    }
}

trait Actor {
    type In;
    type Out;

    fn handle(&self, message: Envelope<Self::In, Self::Out>) -> ();
}

#[derive(Debug)]
struct ActorHandle<In, Out> {
    outbox: Sender<Envelope<In, Out>>,
}

impl<In, Out> ActorHandle<In, Out> {
    fn new(outbox: Sender<Envelope<In, Out>>) -> Self {
        ActorHandle { outbox: outbox }
    }

    fn call(&self, args: In) -> Out {
        let (tx, rx) = mpsc::channel();
        let e = Envelope::new(args, tx);
        self.outbox.send(e).unwrap();
        rx.recv().unwrap()
    }
}

#[derive(Debug)]
struct ActorRunner<A: Actor> {
    actor: A,
    inbox: Receiver<Envelope<A::In, A::Out>>,
}

impl<A: Actor> ActorRunner<A> {
    fn new(actor: A, inbox: Receiver<Envelope<A::In, A::Out>>) -> Self {
        ActorRunner {
            actor: actor,
            inbox: inbox,
        }
    }
}

impl<A: Actor> FnOnce<()> for ActorRunner<A> {
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


// Impl
#[derive(Debug)]
struct EchoActor;

impl Actor for EchoActor {
    type In = i32;
    type Out = Self::In;

    fn handle(&self, message: Envelope<Self::In, Self::Out>) -> () {
        message.output.send(1 / message.input).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let a = EchoActor;
    let h = ActorHandle::new(tx);
    let r = ActorRunner::new(a, rx);
    let t = thread::Builder::new()
        .name("actor".into())
        .spawn(r)
        .unwrap();

    info!("{:?}", h.call(1));
    info!("{:?}", h.call(0));
    t.join().unwrap();
}
