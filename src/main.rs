mod game;

use tokio::signal;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async { fizzbuzz_rs::run().await });

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    handle.abort();
}
