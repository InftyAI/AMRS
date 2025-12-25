pub mod client;

mod router {
    mod random;
    pub mod router;
    pub mod stats;
    mod wrr;
}

mod provider {
    mod fake;
    mod openai;
    pub mod provider;
}
pub mod types {
    pub mod error;
    pub mod responses;
}
