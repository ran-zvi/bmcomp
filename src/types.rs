use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;
pub type Hex = String;


#[derive(Error, Debug)]
pub enum BitMapError {
    #[error("Failed to decode BitMap: {0}")]
    Decode(String),
}