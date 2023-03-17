use std::{process::{exit}};

use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::templates;

pub struct ProjectArgs {
    pub path: String,
    pub name: String,
    pub template: String,
}

impl ProjectArgs {
    pub fn new(path: String, name: String, template: String) -> Self {
        Self {
            path: path,
            name: name,
            template: template,
        }
    }
    pub fn create_project(&mut self) {
        println!(
            "Creating project {} with template {} at path {}",
            style(self.name.clone()).cyan(),
            style(self.template.clone()).blue(),
            style(self.path.clone()).green()
        );
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you really really want to continue?")
            .default(true)
            .show_default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap()
        {
            exit(0);
        }

        templates::from_template(&self);

        println!("Project has been generated");
    }
}
