pub mod net;
pub use net::{NetAPPConfig, NetSDKConfig};
mod structs;
pub use structs::*;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct BotId {
    pub id: String,
    pub platform: String,
}

#[derive(Debug)]
pub enum CallApiError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    ServerError(u16),

    DeserializeFailed(serde_json::Error),
}
