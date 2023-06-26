use log::{debug};
use log4rs;

#[tokio::main]
async fn main() {
    debug!("callme start");
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    for i in 1..100 {
        debug!("execute scan_start_stop() {:?}th time", i);
        btleplugex::start_stop::scan_start_stop().await.unwrap();
    }

    for i in 1..100 {
        debug!("execute connect_disconnect() {:?}th time", i);
        btleplugex::connect_disconnect::connect_disconnect().await.unwrap();
    }
    debug!("callme end");
}
