use std::{fs, path::PathBuf, str::FromStr};

use rustygit::{error::GitError, types::GitUrl, Repository};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// initalize a template in the current directory.
pub fn init_directory(_template: String) -> Result<(), String> {
    if let Some(config_dir) = dirs::config_dir() {
        if !config_dir.exists() {
            if let Err(err) = fs::create_dir(&config_dir) {
                return Err(format!(
                    "Error while creating the config directory! Error: {}",
                    err
                ));
            }
        }

        let template_dir = config_dir.join("projectgen");
        if template_dir.exists() {
            let repo = Repository::new(&template_dir);
            if let Err(err) = repo.fetch_remote("origin") {
                let mut stdout = StandardStream::stdout(ColorChoice::Always);
                if let Err(err_from_set_color) =
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                {
                    println!(
                        "WARNING: Could not set Color for Terminal. Error: {}",
                        err_from_set_color
                    )
                }
                println!("WARNING: Could not fetch in the templates directory. If you have no Internet conection and intended that then ignore this Warning. Error: {}", err)
            }
        } else if let Err(err) = clone_template_git_repo(&template_dir) {
            return Err(format!(
                        "Error while cloning the Git Repo for the first time, this Repo is needed to proceeded. Git Error: {}",
                        err
                        ));
        }
    } else {
        return Err(String::from("Config dir could not be found. Currently you can't use projectgen in this environment. Pls submit a Issue if you need support for your OS."));
    }

    Ok(())
}

fn clone_template_git_repo(path: &PathBuf) -> Result<(), GitError> {
    // RepoBuilder::new().clone("https://github.com/RoBaertschi/projectgen_templates", &path)?;
    Repository::clone(
        GitUrl::from_str("https://github.com/RoBaertschi/projectgen_templates").unwrap(),
        path,
    )?;

    Ok(())
}
