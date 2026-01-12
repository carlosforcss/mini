use bytes::Bytes;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

pub type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
