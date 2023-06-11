use log::{debug};
use log4rs;

fn main() {
    debug!("callme start");
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    btleplugex::callme();
    debug!("callme end");
}
