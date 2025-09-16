use std::path::{Path, PathBuf};

use ignore::gitignore::{Gitignore, GitignoreBuilder};

#[derive(Clone)]
pub struct GitignoreCtx {
    repo_root: Option<PathBuf>,
    repo_exclude: Option<Gitignore>,
    // (directory, matcher for that directory/.gitignore)
    dir_ignores: Vec<(PathBuf, Gitignore)>,
}

impl GitignoreCtx {
    pub fn with_dir(&self, dir: &Path) -> Self {
        let mut next = self.clone();
        let gi_path = dir.join(".gitignore");
        if gi_path.exists() {
            let mut b = GitignoreBuilder::new(dir);
            b.add(&gi_path);
            if let Ok(ig) = b.build() {
                next.dir_ignores.push((dir.to_path_buf(), ig));
            }
        }
        next
    }

    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        if let (Some(root), Some(ig)) = (&self.repo_root, &self.repo_exclude) {
            let rel = path.strip_prefix(root).unwrap_or(path);
            let m = ig.matched_path_or_any_parents(rel, is_dir);
            if m.is_ignore() {
                return true;
            }
        }
        for (dir, ig) in self.dir_ignores.iter().rev() {
            let rel = path.strip_prefix(dir).unwrap_or(path);
            let m = ig.matched_path_or_any_parents(rel, is_dir);
            if m.is_ignore() {
                return true;
            }
        }
        false
    }
}

pub fn build_gitignore_context(start_path: &Path) -> GitignoreCtx {
    let mut ctx = GitignoreCtx { repo_root: None, repo_exclude: None, dir_ignores: Vec::new() };

    #[cfg(not(feature = "no-git"))]
    if let Ok(repo) = git2::Repository::discover(start_path) {
        if let Some(workdir) = repo.workdir() {
            let workdir = workdir.to_path_buf();
            let mut b = GitignoreBuilder::new(&workdir);
            let exclude_path = repo.path().join("info").join("exclude");
            b.add(&exclude_path);
            if let Ok(ig) = b.build() {
                ctx.repo_root = Some(workdir);
                ctx.repo_exclude = Some(ig);
            }
        }
    }

    ctx
}
