use filedescriptor::{FileDescriptor, Pipe};

use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub(crate) static ref EVENT_QUEUE: Arc<EventQueue> = Arc::new(EventQueue::new().expect("failed to create EventQueue"));
}

pub(crate) struct EventQueue {
    write: Mutex<FileDescriptor>,
    read: Mutex<FileDescriptor>,
}
impl EventQueue {
    pub fn new() -> anyhow::Result<Self> {
        Self::new_impl()
    }
}
impl EventQueue {
    fn new_impl() -> anyhow::Result<Self> {
        let mut pipe = Pipe::new()?;
        pipe.write.set_non_blocking(true)?;
        pipe.read.set_non_blocking(true)?;

        Ok(Self {
            write: Mutex::new(pipe.write),
            read: Mutex::new(pipe.read),
        })
    }
}
