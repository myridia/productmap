//use couch_rs::document::DocumentCollection;
use couch_rs::error::CouchError;
//use couch_rs::types::find::FindQuery;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs as fsx;
use std::path::Path;
use tera::Context;
use tera::Tera;

const DB_HOST: &str = "http://cb.neriene.com";
const DB_NAME: &str = "userdb-7061726164697365";
const DB_NAME_MAIN: &str = "perege";
// https://docs.rs/tera/latest/tera/
// https://docs.rs/couch_rs/latest/couch_rs/error/enum.CouchError.html

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = couch_rs::Client::new(DB_HOST, "paradise", "eroarharsoyst")?;
    let clientm = couch_rs::Client::new(DB_HOST, "captain", "rastentsychon")?;
    let db = client.db(DB_NAME).await?;
    let dbm = clientm.db(DB_NAME_MAIN).await?;
    let templates = "templates/*.html";
    //let langs = dbm.get("languages").await?;
    let _doc: Result<Value, CouchError> = dbm.get("languages").await;
    if _doc.is_ok() {
        let langs = &_doc.unwrap()["langs"];
        //println!("...language: {:?}", langs.as_object().unwrap());

        let static_dirs = ["img", "css", "fonts", "js"];
        let static_files = ["style.css"];
        let pages = [
            "index.html",
            "contact.html",
            "about_us.html",
            "rooms.html",
            "amenities.html",
            "nearby_places.html",
            "booking.html",
            "languages.html",
        ];

        for (_k, v) in langs.as_object().unwrap() {
            //println!("...key: {} value:  {}", k, v);
            let i = v.as_str().unwrap();

            if i != "en" {
                //              continue;
            }

            let _path = format!("pages/{i}");
            let path = Path::new(&_path);

            if !path.exists() {
                //println!("...folder does not exist, create it pages/{0}", i);
                fs::create_dir(_path)?;
            }

            for ii in static_dirs {
                let _dir_target = format!("pages/{i}/{ii}");
                let _dir_source = format!("public/{ii}");
                let _dir_source2 = format!("../../public/{ii}");
                let dir_target = Path::new(&_dir_target);
                let dir_source = Path::new(&_dir_source);
                if dir_target.exists() == false && dir_source.exists() == true {
                    //println!("...symlink dir create : /{0}", &_dir_target);
                    fsx::symlink(&_dir_source2, &_dir_target)?;
                }
            }

            for ii in static_files {
                let _file_target = format!("pages/{i}/{ii}");
                let _file_source2 = format!("../../public/{ii}");
                let _file_source = format!("public/{ii}");
                let file_target = Path::new(&_file_target);
                let path_source = Path::new(&_file_source);
                if file_target.exists() == false && path_source.exists() == true {
                    //println!("...create sysmlink: /{0}", &_file_target);
                    fsx::symlink(&_file_source2, &_file_target)?;
                }
            }

            //let find_all = FindQuery::find_all();
            //let docs = db.find_raw(&find_all).await?;
            let r: Result<Value, CouchError> = db.get(i).await;

            if r.is_ok() {
                let doc: Value = r.unwrap();

                //println!("{:?}", r.is_ok());
                //let doc: Value = db.get(i).await?;

                let tera = match Tera::new(templates) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("Parsing error(s): {}", e);
                        ::std::process::exit(1);
                    }
                };

                let mut context = Context::new();
                context.insert("doc", &doc);
                context.insert("langs", &langs);

                for p in pages {
                    let path = &format!("pages/{i}/{p}");
                    println!("...build page: {path}");
                    let r = tera.render(p, &context)?;
                    let mut output = File::create(path)?;
                    let _w = write!(output, "{}", r);
                }
            }

            //        fsx::symlink("../img", "imgx")?;
            //println!("{}", r);
        }
    }

    Ok(())
}
