extern crate rand;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};
use rand::Rng;

#[derive(Debug)]
struct Actor {
    inbox: Receiver<i32>
}

#[derive(Debug)]
struct ActorRef {
    outbox: Sender<i32>
}

impl Actor {
    fn new() -> (ActorRef, Actor) {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        return (ActorRef{outbox: tx}, Actor{inbox: rx})
    }
}

fn main() {
    let a = Actor::new();
    println!("{:?}", a);
}
