use crate::types::{Result, Hex};

pub trait Encode {
    fn encode(&self) -> Result<Hex>;
}

pub trait Decode {
    fn decode(&mut self) -> Result<String>;
}
