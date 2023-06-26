pub mod abi;
pub mod builder;
pub mod connect_disconnect;
pub mod discover_adapters_peripherals;
pub mod start_stop;
pub mod subscribe_notify_characteristic;

use lazy_static::lazy_static;
use log::{debug, info, error};
use crate::builder::{Builder, thread::SpawnAttach as _, Runtime as _};
//use crate::subscribe_notify_characteristic::subscribe;
use crate::discover_adapters_peripherals::discover;
use std::thread;
use tokio::runtime::{Runtime};

lazy_static! {
    pub static ref MYRUNTIME: Runtime = Builder::new();
}

pub fn callme() {
/*
    debug!("calling subscribe() in thread id: {:?}", thread::current().id());
    match MYRUNTIME.block_on(async { subscribe().await }) {
        Ok(r) => { info!("subscribe returned r: {:?}", r); },
        Err(e) => { error!("subscribe returned error: {:?}", e); }
    }
*/

    debug!("calling discover() in thread id: {:?}", thread::current().id());
    let th = thread::Builder::new()
        .name(String::from("discover thread"))
        .spawn_attach(move || {
            debug!("callng discover under MYRUNTIME in thread id: {:?}", thread::current().id());
            match MYRUNTIME.block_on(async { discover().await }) {
                Ok(_) => { info!("discover() returned success");},
                Err(e) => {error!("discover() returned error: {:?}", e)}
            }
            debug!("exiting discover thread: {:?}", thread::current().id());
    }).unwrap();

    th.join().unwrap();

    debug!("end of callme()");
}
