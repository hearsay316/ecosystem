use anyhow::{Context, Error};
use std::fs;
use std::num::ParseIntError;
use thiserror::Error;
#[allow(dead_code)]
#[derive(Error, Debug)]
pub(crate) enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error:{0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error:{0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Error:{0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error :{0}")]
    Custom(String),
}
#[allow(unused)]
#[derive(Debug)]
struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: i64,
}
fn main() -> Result<(), Error> {
    println!("size of anyhowError is {}", size_of::<Error>());
    println!("size of io::Error is {}", size_of::<std::io::Error>());
    println!("size of ParseIntError is {}", size_of::<ParseIntError>());
    println!(
        "size of serde_json::Error is {}",
        size_of::<serde_json::Error>()
    );
    println!("size of String is {}", size_of::<String>());
    println!("size of MyError is {}", size_of::<MyError>());
    let filename = "./Cargo.toml";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file :{}", filename))?;
    fail_with_error()?;
    Ok(())
}
fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
