use android_logger::{Config/*, FilterBuilder*/};
//use btleplug::platform::init;
use crate::builder::{Builder, RuntimeVM as _, thread::SpawnAttach as _};
use crate::callme;
use crate::connect_disconnect::connect_disconnect;
use crate::discover_services_characteristics::services_characteristics;
use crate::MYRUNTIME;
use crate::start_stop::scan_start_stop;
use crate::subscribe_notifications::subscribe;
use jni::{JNIEnv, JavaVM};
use log::{debug, info, error, LevelFilter};
use std::os::raw::c_void;
use std::thread;

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jni::sys::jint {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Debug)
            .with_tag("btleplugex-jni")
            //.with_filter(FilterBuilder::new().parse("trace,jni-utils::crate=debug,jni::crate=debug").build()),
    );
    if let Ok(env) = vm.get_env() {
        if let Ok(_) = jni_utils::init(&env) {
            btleplug::platform::init(&env).map_or_else(
                |e| error!("failed to add btleplug object {:?}", e),
                |_| {
                        info!("btleplug platform init() successful");
                        if let Ok(_) = Builder::set(vm) {
                            info!("vm set successfully to Builder");
                        } else {
                            error!("failed to set vm object to Builder");
                        }
                    }
            );
        } else {
            error!("jni_utils init() failed");
        }
    } else {
        error!("get_env() failed");
    }
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "system" fn Java_com_example_btleplug_run(_env: JNIEnv) {
    debug!("btleplug example run entry");
    callme();
    debug!("btleplug example run exit");
}

#[no_mangle]
pub extern "system" fn Java_com_example_btleplug_scanStartStop(_env: JNIEnv) {
    debug!("Entered btleplugex scan_start_stop()");
    let th = thread::Builder::new()
        .name(String::from("scan_start_stop thread"))
        .spawn_attach(move || {
            match MYRUNTIME.block_on(async { scan_start_stop().await }) {
                Ok(_) => { info!("scan_start_stop() returned success");},
                Err(e) => {error!("scan_start_stop() returned error: {:?}", e)}
            }
            debug!("exiting thread: {:?}", thread::current().name());
    }).unwrap();

    th.join().unwrap();

    debug!("Exiting btleplugex scan_start_stop()");
}

#[no_mangle]
pub extern "system" fn Java_com_example_btleplug_connectDisconnect(_env: JNIEnv) {
    debug!("Entered btleplugex connect_disconnect()");
    let th = thread::Builder::new()
        .name(String::from("connect_disconnect thread"))
        .spawn_attach(move || {
            match MYRUNTIME.block_on(async { connect_disconnect().await }) {
                Ok(_) => { info!("connect_disconnect() returned success");},
                Err(e) => {error!("connect_disconnect() returned error: {:?}", e)}
            }
            debug!("exiting thread: {:?}", thread::current().name());
    }).unwrap();

    th.join().unwrap();

    debug!("Exiting btleplugex connect_disconnect()");
}

#[no_mangle]
pub extern "system" fn Java_com_example_btleplug_servicesCharacteristics(_env: JNIEnv) {
    debug!("Entered btleplugex services_characteristics()");
    let th = thread::Builder::new()
        .name(String::from("services_characteristics thread"))
        .spawn_attach(move || {
            match MYRUNTIME.block_on(async { services_characteristics().await }) {
                Ok(_) => { info!("services_characteristics() returned success");},
                Err(e) => {error!("services_characteristics() returned error: {:?}", e)}
            }
            debug!("exiting thread: {:?}", thread::current().name());
    }).unwrap();

    th.join().unwrap();

    debug!("Exiting btleplugex services_characteristics()");
}

#[no_mangle]
pub extern "system" fn Java_com_example_btleplug_subscribe(_env: JNIEnv) {
    debug!("Entered btleplugex subscribe()");
    let th = thread::Builder::new()
        .name(String::from("subscribe thread"))
        .spawn_attach(move || {
            match MYRUNTIME.block_on(async { subscribe().await }) {
                Ok(_) => { info!("subscribe() returned success");},
                Err(e) => {error!("subscribe() returned error: {:?}", e)}
            }
            debug!("exiting thread: {:?}", thread::current().name());
    }).unwrap();

    th.join().unwrap();

    debug!("Exiting btleplugex subscribe()");
}
