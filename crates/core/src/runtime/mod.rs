pub(crate) mod log;
mod signal;

use anyhow::Result;
use tokio::runtime::Runtime;

use std::cell::OnceCell;
use std::future::Future;

pub struct AsyncExecutor {
    rt: OnceCell<Runtime>,
}

impl AsyncExecutor {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get(&self) -> tokio::Handle { todo!() }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.rt.get().unwrap().block_on(future)
    }

    pub fn run_main<F: Future<Output = !>>(self, _main: fn() -> F) -> ! {
        todo!()
    }

    fn spawn(&mut self) -> Result<()> {
        todo!()
    }
}

