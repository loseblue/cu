
use tracing::*;

mod log;



fn main() {
    println!("CU start !");

    log::init();

    
    trace!("tracing-trace");

}
