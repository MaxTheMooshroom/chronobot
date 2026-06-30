pub type ExecuteReturn = ();

pub trait Execute {
    fn execute(&self) -> ExecuteReturn;
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait ExecuteAsync {
    async fn execute(&self) -> ExecuteReturn;
}
