mod router {
    mod random;
    pub mod router;
    pub mod stats;
    mod wrr;
}
mod config;
mod client {
    pub mod client;
}
mod provider {
    mod fake;
    mod openai;
    pub mod provider;
}

pub use crate::client::client::Client;
pub use crate::config::{Config, ModelConfig, RoutingMode};
pub use crate::provider::provider::{
    APIError, CreateResponseArgs, CreateResponseReq, CreateResponseRes,
};
