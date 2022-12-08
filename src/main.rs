use std::thread;
use std::time::Duration;
use std::env::{args, Args};

mod http_request_processor;
mod ws_request_processor;

fn main() {
    println!("Starting MDB ...");

    let mut args: Args = args();
    let first = args.nth(0);

    println!("{:?}", first);

    ws_request_processor::process_requests();

    http_request_processor::process_requests();

    println!("Started MDB");

    // Hold on to dear life, don't let the main method exit.
    loop {
        println!("Main Thread is waiting ...");
        thread::sleep(Duration::from_secs(5));
    }
}
