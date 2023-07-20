
mod log;
use tracing::{info};

mod threads;    



fn main() {
    println!("CU start !");
    info!("CU start !");

    let _guard = common::log::init();

    let _threads = threads::thread_init();

    println!("CU end !");
}

