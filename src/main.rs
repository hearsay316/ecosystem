use std::fs;
use std::num::ParseIntError;
use ecosystem::MyError;
use anyhow::{Context, Error};

fn main()->Result<(),Error> {
    println!("size of anyhowError is {}",size_of::<Error>());
    println!("size of io::Error is {}",size_of::<std::io::Error>());
    println!("size of ParseIntError is {}",size_of::<ParseIntError>());
    println!("size of serde_json::Error is {}",size_of::<serde_json::Error>());
    println!("size of String is {}",size_of::<String>());
    println!("size of MyError is {}",size_of::<MyError>());
    let filename = "./Cargo.toml";
    let fd = fs::File::open(filename).with_context(||format!("Can not find file :{}",filename))?;
    fail_with_error()?;
    Ok(())
}
fn fail_with_error() ->Result<(),MyError>{
    Err(MyError::Custom("This is a custom error".to_string()))
}