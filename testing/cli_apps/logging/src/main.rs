use log::{info, warn};      // dep -> log = "0.4.19"

use env_logger;

fn main() {
    
    env_logger::init();

    println!("Hello world");
    

    info!("Starting up");
    warn!("Oops that is a warning");

}

//set RUST_LOG=info
// $ cargo run
