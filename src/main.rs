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
    let templates = "templates/*.html";
    let tera = match Tera::new(templates) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    let r = tera.render("index.html", &context)?;
    let path = &format!("website/public/index.html");
    let mut output = File::create(path)?;
    let _w = write!(output, "{}", r);

    for i in 1..5000 {
        let mut context = Context::new();
        let r = tera.render("index.html", &context)?;
        let path = &format!("website/public/product_{0}.html", i);
        let mut output = File::create(path)?;
        let _w = write!(output, "{}", r);
    }

    Ok(())
}
