use serde::{Deserialize, Serialize};

pub mod auth;
pub mod bot;
pub mod channel;
pub mod message;
pub mod user;

/// traQサーバーのホスト名
/// ex. `q.trap.jp`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TraqHost(pub String);

impl std::fmt::Display for TraqHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
