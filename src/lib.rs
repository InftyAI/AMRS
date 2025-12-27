pub mod client;

mod router {
    mod random;
    pub mod router;
    pub mod stats;
    mod wrr;
}

mod provider {
    mod common;
    mod faker;
    mod openai;
    pub mod provider;
}
pub mod types {
    pub mod chat;
    pub mod error;
    pub mod responses;
}
