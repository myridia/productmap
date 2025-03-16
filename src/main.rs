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

    let _path_index = &format!("website/public/index.html");
    let path_index = Path::new(&_path_index);

    let _path_sku = &format!("website/public/sku");

    let path_sku = Path::new(&_path_sku);
    if !path_sku.exists() {
        fs::remove_dir_all(&_path_sku)?;
        fs::create_dir(_path_sku)?;
    }
    let _path_category = &format!("website/public/category");
    let path_category = Path::new(&_path_category);
    if !path_category.exists() {
        fs::remove_dir_all(&_path_category)?;
        fs::create_dir(_path_category)?;
    }

    let mut v = vec!["".to_string()];

    let mut context = Context::new();
    for i in 1..120 {
        let mut context = Context::new();
        v.push(i.to_string());
        context.insert("sku", &i.to_string());
        let r = tera.render("sku.html", &context)?;
        let path = &format!("website/public/sku/{0}.html", i);
        let mut output = File::create(path)?;
        let _w = write!(output, "{}", r);
    }

    let mut context1 = Context::new();
    for i in 1..12 {
        let mut context1 = Context::new();
        let r = tera.render("category.html", &context)?;
        let path = &format!("website/public/category/{0}.html", i);
        let mut output = File::create(path)?;
        let _w = write!(output, "{}", r);
    }

    let mut context2 = Context::new();
    context2.insert("v", &v);
    let r = tera.render("index.html", &context2)?;
    let mut output = File::create(path_index)?;
    let _w = write!(output, "{}", r);
    Ok(())
}
