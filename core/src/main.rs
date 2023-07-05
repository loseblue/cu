
use tracing::{info};
mod log;
mod threads;



fn main() {
    println!("CU start !");
    info!("CU start !");

    let _guard = log::init();

    threads::thread_init();

    println!("CU end !");
}

