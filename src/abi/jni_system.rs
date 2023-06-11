use android_logger::{Config/*, FilterBuilder*/};
//use btleplug::platform::init;
use crate::builder::{Builder, RuntimeVM as _};
use crate::callme;
use jni::{JNIEnv, JavaVM};
use log::{debug, info, error, LevelFilter};
use std::os::raw::c_void;

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
