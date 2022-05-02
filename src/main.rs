use std::{future::Future, time::Duration};

use futures::future::join_all;
use rand::prelude::*;
use tokio::{time::sleep, sync::watch::{self, Receiver}, select, signal};

#[derive(Clone, Debug)]
enum Command {
    Stop,
}

#[tokio::main]
async fn main() {
    let mut done = false;
    let (tx, rx) = watch::channel(Command::Stop);

    let workers = start_workers(3, rx);
    let workers_future = join_all(workers);

    tokio::pin!(workers_future);
    
    while !done {
        select! {
            _ = &mut workers_future => {
                done = true;
            },
            _ = signal::ctrl_c() => {
                tx.send(Command::Stop).expect("unable to send command to channel");
            }
        }
    }
}

fn start_workers(count: usize, rx: Receiver<Command>) -> Vec<impl Future<>> {
    let mut futures = Vec::new();
    for id in 1..=count {
        futures.push(worker(id, rx.clone()));
    }
    futures
}

async fn worker(id: usize, mut rx: Receiver<Command>) {
    loop {
        let mut rng = rand::thread_rng();
        let t = rng.gen_range(2000..=5000);

        select! {
            _ = sleep(Duration::from_millis(t)) => (),
            _ = rx.changed() => {
                println!("{}: stopping", id);
                break;
            }
        }

        println!("{}: tick", id);
    }
}