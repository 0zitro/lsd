//! This module defines the [TreePath]. To set it up from [Cli], a [Config] and its
//! [Default] value, use the [configure_from](Configurable::configure_from) method.

use super::Configurable;
use crate::app::Cli;
use crate::config_file::Config;

use serde::Deserialize;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TreePathType {
    #[default]
    None,
    Absolute,
    Relative,
}

impl TreePathType {
    fn from_arg_str(v: &str) -> Self {
        match v {
            "none" => Self::None,
            "absolute" => Self::Absolute,
            "relative" => Self::Relative,
            other => unreachable!("Invalid value '{other}' for 'tree-path'"),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum TreePathScope {
    #[default]
    Root,
    All,
}

impl TreePathScope {
    fn from_arg_str(v: &str) -> Self {
        match v {
            "root" => Self::Root,
            "all" => Self::All,
            other => unreachable!("Invalid value '{other}' for 'tree-path-scope'"),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct TreePath {
    pub kind: TreePathType,
    pub scope: TreePathScope,
}

impl Configurable<Self> for TreePath {
    fn from_cli(cli: &Cli) -> Option<Self> {
        let kind = cli.tree_path.as_deref().map(TreePathType::from_arg_str);
        let scope = cli
            .tree_path_scope
            .as_deref()
            .map(TreePathScope::from_arg_str);
        match (kind, scope) {
            (None, None) => None,
            (k, s) => Some(Self {
                kind: k.unwrap_or_default(),
                scope: s.unwrap_or_default(),
            }),
        }
    }

    fn from_config(config: &Config) -> Option<Self> {
        let kind = config.tree_path.as_deref().map(TreePathType::from_arg_str);
        let scope = config
            .tree_path_scope
            .as_deref()
            .map(TreePathScope::from_arg_str);

        match (kind, scope) {
            (None, None) => None,
            (k, s) => Some(Self {
                kind: k.unwrap_or_default(),
                scope: s.unwrap_or_default(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flags;
    use crate::{Config, app::Cli};
    use clap::Parser;

    // Contract:
    // - configure_from combines CLI, env (n/a), and config; CLI takes precedence.
    // - Default when neither set is (None, Root).
    // - If only scope provided, kind defaults to None.
    // - If only kind provided, scope defaults to Root.

    #[test]
    fn tree_path_default_when_unset() {
        let argv = ["lsd"]; // no cli args
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = Config::with_none(); // no config
        let flags = Flags::configure_from(&cli, &cfg).unwrap();
        assert_eq!(TreePathType::None, flags.tree_path.kind);
        assert_eq!(TreePathScope::Root, flags.tree_path.scope);
    }

    #[test]
    fn tree_path_from_config_only_kind_defaults_scope() {
        let argv = ["lsd"]; // no cli args
        let cli = Cli::try_parse_from(argv).unwrap();
        let mut cfg = Config::with_none();
        cfg.tree_path = Some("absolute".into());
        let flags = Flags::configure_from(&cli, &cfg).unwrap();
        assert_eq!(TreePathType::Absolute, flags.tree_path.kind);
        assert_eq!(TreePathScope::Root, flags.tree_path.scope);
    }

    #[test]
    fn tree_path_from_config_both() {
        let argv = ["lsd"]; // no cli args
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = serde_yaml::from_str::<Config>(r#"
            tree-path: relative
            tree-path-scope: all
        "#).unwrap();
        let flags = Flags::configure_from(&cli, &cfg).unwrap();
        assert_eq!(TreePathType::Relative, flags.tree_path.kind);
        assert_eq!(TreePathScope::All, flags.tree_path.scope);
    }

    #[test]
    fn tree_path_from_cli_overrides_config() {
        let argv = [
            "lsd",
            "--tree-path",
            "absolute",
            "--tree-path-scope",
            "all",
        ];
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = serde_yaml::from_str::<Config>(r#"
            tree-path: relative
            tree-path-scope: root
        "#).unwrap();
        let flags = Flags::configure_from(&cli, &cfg).unwrap();
        assert_eq!(TreePathType::Absolute, flags.tree_path.kind);
        assert_eq!(TreePathScope::All, flags.tree_path.scope);
    }

    #[test]
    fn tree_path_scope_from_cli_and_kind_from_config() {
        let argv = ["lsd", "--tree-path-scope", "all"]; // only scope via CLI
        let cli = Cli::try_parse_from(argv).unwrap();
        let cfg = serde_yaml::from_str::<Config>(r#"
            tree-path: absolute
        "#).unwrap();
        let flags = Flags::configure_from(&cli, &cfg).unwrap();
        assert_eq!(TreePathType::None, flags.tree_path.kind);
        assert_eq!(TreePathScope::All, flags.tree_path.scope);
    }
}
