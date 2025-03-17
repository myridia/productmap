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

    let _path_home = &format!("website/public/home.html");
    let path_home = Path::new(&_path_home);

    let _path_product = &format!("website/public/product");

    let path_product = Path::new(&_path_product);
    if path_product.exists() {
        fs::remove_dir_all(&_path_product)?;
    }
    if !path_product.exists() {
        fs::create_dir(_path_product)?;
    }
    let _path_category = &format!("website/public/category");
    let path_category = Path::new(&_path_category);
    if path_category.exists() {
        fs::remove_dir_all(&_path_category)?;
    }
    if !path_category.exists() {
        fs::create_dir(_path_category)?;
    }

    let mut categories = vec![];

    for i in 1..8 {
        let mut products = vec![];
        categories.push(format!("/category/category_{i}.html"));
        for ii in 1..11 {
            let mut context_product = Context::new();
            products.push(format!("/product/product_{i}_{ii}.html"));
            context_product.insert("product", &format!("product_{i}_{ii}.html"));
            let r = tera.render("product.html", &context_product)?;
            let path = &format!("website/public/product/product_{0}_{1}.html", i, ii);
            let mut output = File::create(path)?;
            let _w = write!(output, "{}", r);
        }

        let mut context_category = Context::new();
        context_category.insert("products", &products);
        let r = tera.render("category.html", &context_category)?;
        let path = &format!("website/public/category/category_{0}.html", i);
        let mut output = File::create(path)?;
        let _w = write!(output, "{}", r);
    }

    let mut context_home = Context::new();
    context_home.insert("categories", &categories);
    let r = tera.render("home.html", &context_home)?;
    let mut output = File::create(path_home)?;
    let _w = write!(output, "{}", r);

    Ok(())
}
