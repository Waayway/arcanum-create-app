use std::{fs::create_dir_all, path::PathBuf};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};

mod projectargs;
mod templates;
use projectargs::ProjectArgs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: PathBuf,
    #[arg(short, long)]
    template: Option<String>,
}

fn main() {
    let args = Args::parse();
    if !args.path.exists() {
        create_dir_all(args.path.as_path()).unwrap();
    }
    let binding = args.path.as_path().canonicalize().unwrap();
    let folder_name = binding.file_name().unwrap().to_str().unwrap();
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default(folder_name.to_string())
        .interact_text()
        .unwrap();


    let mut template: String = "".to_string();
    if args.template.is_none() {
        let template_options = &[
            "default-html-rust",
            "rust-html-scss-js",
            "rust-html-scss-ts",
        ];
    
        template = template_options[Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a template")
            .default(0)
            .items(&template_options[..])
            .interact()
            .unwrap()].to_string();
    } else if let Some(tmplt) = args.template {
        template = tmplt.clone();
    }

    let mut project = ProjectArgs::new(
        args.path.as_path().to_str().unwrap().to_owned(),
        name,
        template.to_string(),
    );
    
    project.create_project();
}
