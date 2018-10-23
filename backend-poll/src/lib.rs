#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]

extern crate notify_backend as backend;

use backend::prelude::*;
use backend::Buffer;
use std::sync::Arc;

const BACKEND_NAME: &str = "poll";

#[derive(Debug)]
pub struct Backend {
    buffer: Buffer,
    reg: Arc<MioRegistration>,
    watches: Vec<PathBuf>,
}

impl NotifyBackend for Backend {
    fn name() -> &'static str {
        BACKEND_NAME
    }

    fn new(watches: Vec<PathBuf>) -> NewBackendResult {
        let (reg, readiness) = MioRegistration::new2();
        readiness.set_readiness(MioReady::readable());

        Ok(Box::new(Self {
            buffer: Buffer::default(),
            reg: Arc::new(reg),
            watches,
        }))
    }

    fn capabilities() -> Vec<Capability> {
        vec![Capability::FollowSymlinks, Capability::WatchFiles]
    }

    fn driver(&self) -> Box<Evented> {
        Box::new(self.reg.clone())
    }
}

impl Drop for Backend {
    fn drop(&mut self) {}
}

impl Stream for Backend {
    type Item = StreamItem;
    type Error = StreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.buffer.poll()
    }
}
