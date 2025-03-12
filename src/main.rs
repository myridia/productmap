use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs as fsx;
use std::path::Path;
use tera::Context;
use tera::Tera;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("...main");
    Ok(())
}
