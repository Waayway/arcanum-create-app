use std::{fs, process::exit, path::Path};

use rust_embed::RustEmbed;

use crate::projectargs::ProjectArgs;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates/default-html-rust"]
struct DefaultHtmlRust;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates/rust-html-scss-js"]
struct RustHtmlScssJs;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/templates/rust-html-scss-ts"]
struct RustHtmlScssTs;

pub fn from_template(projectargs: &ProjectArgs) {
    let template: &str = &projectargs.template;
    match template {
        "default-html-rust" => write_folder(DefaultHtmlRust, projectargs),
        "rust-html-scss-js" => write_folder(RustHtmlScssJs, projectargs),
        "rust-html-scss-ts" => write_folder(RustHtmlScssTs, projectargs),
        _ => {}
    }
}


fn write_folder<T>(_: T, projectargs: &ProjectArgs) where T: RustEmbed {
    fs::create_dir_all(projectargs.path.clone()).unwrap_or_else(|err| {
        eprintln!("ERROR: Couldn't create directory because of {err}");
        exit(0);
    });
    let path_obj = Path::new(&projectargs.path);

    for i in T::iter() {
        let new_path = path_obj.join(&i.clone().to_string());
        fs::create_dir_all(new_path.parent().unwrap()).unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't create directory {new_path:?}")
        });
        let data = match T::get(&i) {
            None => {eprintln!("ERROR: Couldn't get data from file"); continue;},
            Some(data) => data
        };
        if i == "Cargo.toml" {
            let file_content = match String::from_utf8(data.data.to_vec()) {
                Err(err) => {eprintln!("ERROR: Couldn't open Cargo.toml {err}"); continue;}
                Ok(data) => data,
            };
            let mut new_file = String::new();
            let lines = file_content.lines();
            for i in lines {
                if i.starts_with("name = ") {
                    new_file += &format!("name = \"{}\"\n", projectargs.name);
                } else {
                    new_file += &(i.to_owned() + "\n");
                }
            }
            fs::write(new_path.clone(), new_file.as_bytes()).unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't write file {new_path:?}, because of {err}");
            });
        } else {
            fs::write(new_path.clone(), data.data).unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't write file {new_path:?}, because of {err}");
            });
        }
    }
}