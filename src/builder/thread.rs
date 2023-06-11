//! Extended the standard thread library with new spwan and attach
//! capabilites. std::thread object creates thread and attaches it
//! to VM instance (android) .Detachment with VM happens on thread
//! exit.
//! ```rust
//! use crate::builder::thread::{SpawnAttach _};
//! thread::Builder::new().spawn_attach(move || {});
//! ```
use crate::builder::{Builder, RuntimeVM as _};
use crate::thread::JoinHandle;
use std::io;

pub trait SpawnAttach {
    fn spawn_attach<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static;
}

impl SpawnAttach for std::thread::Builder {
    fn spawn_attach<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        self.spawn(move || {
            Builder::attach();
            f()
        })
    }
}
