
use serde::{Serialize, Deserialize};

pub mod core;
pub mod traits;
pub mod behaviour;

pub use crate::core::*;
pub use crate::traits::*;
pub use behaviour::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    Name: String,
    Age: u8,
    Gender: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Dog {
    Name: String,
    Age: u8,
    Type: String,
}