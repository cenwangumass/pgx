/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use eyre::eyre;
use std::{io::Write, path::PathBuf, str::FromStr};

use crate::CommandExecute;

/// Create a new extension crate
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct New {
    /// The name of the extension
    name: String,
    /// Create a background worker template
    #[clap(long, short)]
    bgworker: bool,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for New {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        validate_extension_name(&self.name)?;
        let path = PathBuf::from_str(&format!("{}/", self.name)).unwrap();
        create_crate_template(path, &self.name, self.bgworker)
    }
}

fn validate_extension_name(extname: &str) -> eyre::Result<()> {
    for c in extname.chars() {
        if !c.is_alphanumeric() && c != '_' && !c.is_lowercase() {
            return Err(eyre!("Extension name must be in the set of [a-z0-9_]"));
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path, name))]
pub(crate) fn create_crate_template(
    path: PathBuf,
    name: &str,
    is_bgworker: bool,
) -> eyre::Result<()> {
    create_directory_structure(&path)?;
    create_control_file(&path, name)?;
    create_cargo_toml(&path, name)?;
    create_dotcargo_config(&path, name)?;
    create_lib_rs(&path, name, is_bgworker)?;
    create_git_ignore(&path, name)?;

    Ok(())
}

fn create_directory_structure(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut src_dir = path.clone();

    src_dir.push("src");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push(".cargo");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push("sql");
    std::fs::create_dir_all(&src_dir)
}

fn create_control_file(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(format!("{}.control", name));
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/control"), name = name).as_bytes())?;

    Ok(())
}

fn create_cargo_toml(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("Cargo.toml");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/cargo_toml"), name = name).as_bytes())?;

    Ok(())
}

fn create_dotcargo_config(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".cargo");
    filename.push("config");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(include_bytes!("../templates/cargo_config"))?;

    Ok(())
}

fn create_lib_rs(path: &PathBuf, name: &str, is_bgworker: bool) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("src");
    filename.push("lib.rs");
    let mut file = std::fs::File::create(filename)?;

    if is_bgworker {
        file.write_all(
            &format!(include_str!("../templates/bgworker_lib_rs"), name = name).as_bytes(),
        )?;
    } else {
        file.write_all(&format!(include_str!("../templates/lib_rs"), name = name).as_bytes())?;
    }

    Ok(())
}

fn create_git_ignore(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".gitignore");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(include_bytes!("../templates/gitignore"))?;

    Ok(())
}
