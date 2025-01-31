use std::{fs, path::PathBuf};

use crate::config::Workspace;
use anyhow::{bail, Result};

const CONFIG_FILE: &str = "Crux.toml";

pub fn read_config() -> Result<Workspace> {
    let path = PathBuf::from(CONFIG_FILE);
    if let Ok(file) = &fs::read_to_string(path) {
        let mut workspace: Workspace = toml::from_str(file)?;

        let all_cores = workspace.cores.keys().cloned().collect::<Vec<_>>();
        if all_cores.len() == 0 {
            bail!("{CONFIG_FILE}: no cores defined");
        }

        for (name, core) in &mut workspace.cores {
            core.name = name.to_string();
            if !core.source.exists() {
                bail!(
                    "{CONFIG_FILE}: core ({name}) source directory ({path}) does not exist",
                    path = core.source.display()
                );
            }
        }

        if let Some(shells) = &mut workspace.shells {
            for (name, shell) in shells {
                shell.name = name.to_string();
                if !shell.source.exists() {
                    bail!(
                        "{CONFIG_FILE}: shell ({name}) source directory ({path}) does not exist",
                        path = shell.source.display()
                    );
                }
                if !shell.cores.iter().all(|core| all_cores.contains(core)) {
                    bail!("{CONFIG_FILE}: shell ({name}) references a core that does not exist");
                }
            }
        }

        Ok(workspace)
    } else {
        Ok(Workspace::default())
    }
}

pub fn write_config(workspace: &Workspace) -> Result<()> {
    let path = PathBuf::from(CONFIG_FILE);
    let toml = toml::to_string(workspace)?;
    fs::write(path, toml)?;
    Ok(())
}
