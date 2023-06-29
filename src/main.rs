
use tracing::{info};
mod log;
mod core;



fn main() {
    println!("CU start !");
    info!("CU start !");

    let _guard = log::init();

    core::thread_init();
        
 

    println!("CU end !");
}

