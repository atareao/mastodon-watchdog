mod config;
mod feedback;
mod mastodon;
mod matrix;
mod message;
mod zinc;

pub use config::Config;
pub use feedback::Feedback;
pub use zinc::Zinc;
pub use mastodon::Mastodon;
pub use matrix::Matrix;
pub use message::{
    check_key,
    check_comment,
};

pub type Error = Box<dyn std::error::Error>;


