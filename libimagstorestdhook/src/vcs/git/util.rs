//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

use git2::Repository;

use vcs::git::error::{GitHookError as GHE, GitHookErrorKind as GHEK};

pub fn mkrepo(store: &Store) -> Result<()> {
    let mut opts = RepositoryInitOpts::new();
    opts.bare(false);
    opts.no_reinit(true);
    opts.mkdir(false);
    opts.external_template(false);
    Repository::init_opts(store.path(), &opts)
        .map(|_| ())
        .map_err(|e| GHE::new(GHEK::MkRepo, Some(Box::new(e))))
}

pub fn hasrepo(store: &Store) -> bool {
    unimplemented!()
}

