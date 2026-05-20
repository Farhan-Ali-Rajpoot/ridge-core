


use crate::core::router::io::request::Connection;
use crate::core::pingora::PingoraAdapter;


impl<T> PingoraAdapter<T> where T: Send + Sync + 'static { 
    pub async fn run_middlewares(&self, req: &mut Connection) -> pingora::Result<bool> {
        Ok(true)
    }
}
