use anyhow::Result;
use tokio::runtime::Runtime;

use std::future::Future;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct ExecutorAsync {
    rt: Mutex<Runtime>,
}

static GLOBAL_EXECUTOR: OnceLock<ExecutorAsync> = OnceLock::new();

impl ExecutorAsync {
    fn new() -> Self {
        ExecutorAsync { rt: Mutex::new(Runtime::new().unwrap()) }
    }

    pub fn get<'a>() -> MutexGuard<'a, Runtime> {
        GLOBAL_EXECUTOR.get_or_init(Self::new).rt.lock().unwrap()
    }

    pub fn block_on<F: Future>(future: F) -> F::Output {
        Self::get().handle().block_on(future)
    }

    pub fn run_main<F: Future<Output = Result<!>>>(main: F) -> Result<!> {
        Self::block_on(main)
    }

    pub fn spawn<F>(task: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::get().handle().spawn(task)
    }
}

