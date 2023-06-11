use crate::builder::RuntimeVM;
use crate::lazy_static;
use jni::objects::GlobalRef;
use jni::JavaVM;
use log::{error, info, trace};
use once_cell::sync::OnceCell;

lazy_static! {
    static ref GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();
    static ref GLOBAL_CLASS_LOADER: OnceCell<GlobalRef> = OnceCell::new();
}

impl super::Runtime for super::Builder {
    fn new() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(Self::TASKS)
            .max_blocking_threads(Self::WORKERS)
            .on_thread_start(|| {
                trace!("Runtime Builder on_thread_start()");
                Self::attach();
            })
            .on_thread_stop(|| {
                trace!("Runtime Builder on_thread_stop()");
                Self::detach();
            })
            .build()
            .unwrap()
    }
}

impl super::RuntimeVM for super::Builder {
    type T = JavaVM;
    fn set(vm: Self::T) -> Result<(), Self::T> {
        let rv = GLOBAL_JVM.set(vm);
        if rv.is_ok() {
            if let Some(vm) = GLOBAL_JVM.get() {
                if let Ok(env) = vm.attach_current_thread_permanently() {
                    let thread = env
                        .call_static_method(
                            "java/lang/Thread",
                            "currentThread",
                            "()Ljava/lang/Thread;",
                            &[],
                        )
                        .unwrap()
                        .l()
                        .unwrap();
                    let class_loader = env
                        .call_method(
                            thread,
                            "getContextClassLoader",
                            "()Ljava/lang/ClassLoader;",
                            &[],
                        )
                        .unwrap()
                        .l()
                        .unwrap();
                    if let Ok(_) = GLOBAL_CLASS_LOADER.set(env.new_global_ref(class_loader).unwrap()) {
                        info!("class_loader set successfully");
                    } else {
                        error!("failed to set class_loader");
                    }
                }
            }
        }
        rv
    }

    fn attach() {
        if let Some(vm) = GLOBAL_JVM.get() {
            let env = vm.attach_current_thread_permanently().unwrap();
            if let Some(class_loader) = GLOBAL_CLASS_LOADER.get() {
                let thread = env
                    .call_static_method(
                        "java/lang/Thread",
                        "currentThread",
                        "()Ljava/lang/Thread;",
                        &[],
                    )
                    .unwrap()
                    .l()
                    .unwrap();
                env.call_method(
                    thread,
                    "setContextClassLoader",
                    "(Ljava/lang/ClassLoader;)V",
                    &[class_loader.as_obj().into()],
                )
                .unwrap();
            }
        }
        info!("Attached to Java VM");
    }

    fn detach() {
        info!("RuntimeVM detach()");
        if let Some(vm) = GLOBAL_JVM.get() {
            vm.detach_current_thread();
            info!("Detached from Java VM");
        }
    }
}
