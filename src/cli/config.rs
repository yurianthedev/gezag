use std::{
    env, fs,
    path::{Path, PathBuf},
};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CliConfig {
    pub librarian: CliLibrarians,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case", tag = "librarian-kind")]
pub enum CliLibrarians {
    Local { registry: LocalRegistryConfig },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalRegistryConfig {
    pub location: String,
}

pub struct CliConfigProvider {
    app_home_path: PathBuf,
    config_file_path: PathBuf,
}

impl CliConfigProvider {
    pub fn app_home_path(&self) -> &Path {
        self.app_home_path.as_ref()
    }

    pub fn config_file_path(&self) -> &Path {
        self.config_file_path.as_ref()
    }

    pub fn read(&self) -> Result<CliConfig, anyhow::Error> {
        let contents = fs::read_to_string(&self.config_file_path)?;
        let cli_config = serde_yaml::from_str(&contents)?;
        Ok(cli_config)
    }

    pub fn write(&self, config: CliConfig) -> Result<(), anyhow::Error> {
        let config_str = serde_yaml::to_string(&config)?;
        fs::write(&self.config_file_path, config_str)?;
        Ok(())
    }

    /// Throws if none of the options from where it tries to fetch it works.
    fn gen_app_home_path(home_var_name: &str, app_dir_name: &str) -> PathBuf {
        env::var_os(home_var_name) // Look for an env var.
            .map(|p| Path::new(&p).to_owned())
            .or_else(|| {
                UserDirs::new().map(|uds| uds.home_dir().to_owned().join(Path::new(app_dir_name)))
            }) // Or uses the default, at the home dir.
            .expect("Unable to get a config directory")
    }
}

impl Default for CliConfigProvider {
    fn default() -> Self {
        // TODO: Group those defaults elsewhere.
        let default_app_dir_name = ".gezag";
        let default_home_var_name = "GEZAG_HOME";
        let default_file_name = format!("{}.{}", "config", "yaml");

        let app_home_path = Self::gen_app_home_path(default_home_var_name, default_app_dir_name);
        let config_file_path = app_home_path.join(Path::new(&default_file_name));

        Self {
            app_home_path,
            config_file_path,
        }
    }
}
