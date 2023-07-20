#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Disconnect error")]
    ReqwestError(String),
    #[error("Disconnect error")]
    DeserializeError(String),
}
