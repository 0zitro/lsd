use crate::app::Cli;
use crate::config_file::Config;

use super::Configurable;

#[derive(Clone, Debug, Default)]
pub struct GitIgnore(pub bool);

impl Configurable<Self> for GitIgnore {
    fn from_cli(cli: &Cli) -> Option<Self> {
        Some(GitIgnore(cli.gitignore))
    }

    fn from_config(config: &Config) -> Option<Self> {
        config.gitignore.map(GitIgnore)
    }
}
