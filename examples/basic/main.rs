use std::thread;

mod client;
mod server;

fn main() {
    thread::spawn(server::run);
    client::run();
}
