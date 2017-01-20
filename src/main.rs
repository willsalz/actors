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

trait Actor {
    type In;
    type Out;

    fn handle(&self, message: Envelope<Self::In, Self::Out>) -> ();
    fn inbox(&self) -> Sender<Self::In>;
}

#[derive(Debug)]
struct ActorHandle<A: Actor> {
    actor: A,
}

impl<A: Actor> ActorHandle<A> {
    fn call(&self, args: A::In) -> A::Out {
        unimplemented!();
    }
}

// Impl
#[derive(Debug)]
struct EchoActor {
    inbox: Sender<i32>,
}

impl Actor for EchoActor {
    type In = i32;
    type Out = i32;

    fn handle(&self, message: Envelope<Self::In, Self::Out>) -> () {
        message.output.send(message.input).unwrap();
    }

    fn inbox(&self) -> Sender<Self::In> {
        return self.inbox.clone();
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let a = EchoActor { inbox: tx };
    let h = ActorHandle { actor: a };
    info!("{:?}", h.call(42));
}
