use std::{fs, path::PathBuf};

use git2::{build::RepoBuilder, Repository};

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
            let repo = Repository::open(template_dir)
                .expect("Error: Could not find Repo in the templates dir.");

            let mut remote = repo
                .find_remote("origin")
                .expect("Error: Could not find Remote origin in template dir.");

            remote.fetch(&["master"], None, None);
            remote.disconnect();
        } else {
            if let Err(err) = clone_template_git_repo(template_dir) {
                return Err(format!(
                    "Cloning the templates repo failed. Git2 Error: {}",
                    err
                ));
            }
        }
    } else {
        return Err(String::from("Config dir could not be found. Currently you can't use projectgen in this environment. Pls submit a Issue if you need support for your OS."));
    }

    Ok(())
}

fn clone_template_git_repo(path: PathBuf) -> Result<(), git2::Error> {
    RepoBuilder::new().clone("https://github.com/RoBaertschi/projectgen_templates", &path)?;

    Ok(())
}
