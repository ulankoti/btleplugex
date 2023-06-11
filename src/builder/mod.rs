/// Builder module to implement the VM interfaces. Only Android
/// VM needs this facility to attach the native threads to VM.
/// For all the remaining OS's use the default trait implementa
/// ion.
/// # Example
/// ```rust
/// 	use crate::builder::{Builder, RuntimeVM as _};
/// 	Builder::set(vm);
/// 	Builder::attach();
/// ```
pub struct Builder;
pub mod thread;

/// Implement android runtime, runtime VM interfaces.
#[cfg(target_os = "android")]
pub mod jni;

/// Create run time (TOKIO) to handle the blocking call's and
/// spawn the thread to run blocking task. TOKIO run time prov
/// ides ability to call user provided closuer thread start,
/// thread stop. Use this to attach the blocking worker thread
/// to VM (like android).
pub trait Runtime {
    const WORKERS: usize = 20;
    const TASKS: usize = 10;
    fn new() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(Self::TASKS)
            .max_blocking_threads(Self::WORKERS)
            .build()
            .unwrap()
    }
}

///
/// To make standard threads to work with android VM. Provide
/// the interfaces to store the VM context, attach the thread
/// ,detach thread from the android VM intance.
///
pub trait RuntimeVM {
    type T;
    fn set(_: Self::T) -> Result<(), Self::T> {
        Ok(())
    }
    fn attach() {}
    fn detach() {}
}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "linux",
    target_os = "windows"
))]
impl Runtime for Builder {}

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "linux",
    target_os = "windows"
))]
impl RuntimeVM for Builder {
    type T = ();
}
