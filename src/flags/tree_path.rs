//! Tree path display options for tree layout.

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
        Some(Self {
            kind: TreePathType::from_arg_str(&cli.tree_path),
            scope: TreePathScope::from_arg_str(&cli.tree_path_scope),
        })
    }

    fn from_config(config: &Config) -> Option<Self> {
        let kind =
            config.tree_path
                .as_deref()
                .map(TreePathType::from_arg_str);
        let scope =
            config.tree_path_scope
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
