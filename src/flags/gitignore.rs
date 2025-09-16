use crate::app::Cli;
use crate::config_file::Config;

use super::Configurable;

#[derive(Clone, Debug, Default)]
pub struct GitIgnore(pub bool);

impl Configurable<Self> for GitIgnore {
    fn from_cli(cli: &Cli) -> Option<Self> {
        if cli.gitignore {
            Some(GitIgnore(true))
        } else {
            None
        }
    }

    fn from_config(config: &Config) -> Option<Self> {
        config.gitignore.map(GitIgnore)
    }
}

#[cfg(test)]
mod test {
    use super::GitIgnore;
    use crate::app::Cli;
    use crate::config_file::Config;
    use crate::flags::Configurable;
    use clap::Parser;

    #[test]
    fn test_configuration_from_none() {
        let argv = ["lsd"]; // no CLI flag
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = Config::with_none();
        let v = <GitIgnore as Configurable<GitIgnore>>::configure_from(&cli, &cfg);
        assert_eq!(v.0, false);
    }

    #[test]
    fn test_configuration_from_args() {
        let argv = ["lsd", "--gitignore"]; // CLI wins
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = Config::with_none();
        let v = <GitIgnore as Configurable<GitIgnore>>::configure_from(&cli, &cfg);
        assert_eq!(v.0, true);
    }

    #[test]
    fn test_configuration_from_config() {
        let argv = ["lsd"]; // not provided on CLI
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.gitignore = Some(true);
        let v = <GitIgnore as Configurable<GitIgnore>>::configure_from(&cli, &c);
        assert_eq!(v.0, true);
    }

    #[test]
    fn test_cli_overrides_config() {
        let argv = ["lsd", "--gitignore"]; // CLI true, config false
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut c = Config::with_none();
        c.gitignore = Some(false);
        let v = <GitIgnore as Configurable<GitIgnore>>::configure_from(&cli, &c);
        assert_eq!(v.0, true);
    }
}
