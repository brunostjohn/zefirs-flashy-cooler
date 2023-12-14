use std::future::Future;

pub trait DisplayCooler: Sync + Send {
    fn initialise(&mut self) -> impl Future<Output = anyhow::Result<()>> + '_ + Sync + Send;
    fn close(&mut self) -> impl Future<Output = anyhow::Result<()>> + '_ + Sync + Send;
    fn reopen(&mut self) -> impl Future<Output = anyhow::Result<()>> + Sync + Send;
    fn send_image<'a, 'b>(
        &'a mut self,
        image: &'b [u8],
    ) -> impl Future<Output = anyhow::Result<()>> + Sync + Send;
}
