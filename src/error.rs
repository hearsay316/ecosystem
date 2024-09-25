use std::io::Error;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum MyError{
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error:{0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error:{0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Error:{0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error :{0}")]
    Custom(String)
}
#[allow(unused)]
#[derive(Debug)]
struct BigError{
    a:String,
    b:Vec<String>,
    c:[u8;64],
    d:i64
}
// impl From<std::io::Error> for MyError{
//     fn from(value: Error) -> Self {
//         MyError::Custom(value.to_string())
//     }
// }