
use tracing::{info};
mod log;
mod threads;



fn main() {
    println!("CU start !");
    info!("CU start !");

    let _guard = log::init();

    threads::thread_init();
    std::thread::sleep(std::time::Duration::from_millis(5100));

    println!("CU end !");
}

