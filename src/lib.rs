mod router {
    mod random;
    pub mod router;
    mod weight;
}
mod client {
    pub mod client;
}
mod provider {
    mod openai;
    pub mod provider;
}

pub mod config;
pub use crate::client::client::Client;
