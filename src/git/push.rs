use git2::{Error, Repository};
use git2_credentials::ui4dialoguer::CredentialUI4Dialoguer;
use git2_credentials::CredentialHandler;

pub fn push_branch(repo: &Repository, branch: &str, remote_name: &str) -> Result<(), Error> {
    let mut origin = repo.find_remote(remote_name)?;

    let mut cb = git2::RemoteCallbacks::new();
    let git_config = git2::Config::open_default()?;
    // Prepare callbacks.
    let mut ch = CredentialHandler::new_with_ui(git_config, Box::new(CredentialUI4Dialoguer {}));

    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));

    let mut po = git2::PushOptions::new();

    po.remote_callbacks(cb);
    let result = origin.push(&[&ref_by_branch(branch)], Some(&mut po));
    log::debug!("Push result {:?}", result);

    Ok(())
}

fn ref_by_branch(branch: &str) -> String {
    format!("refs/heads/{}:refs/heads/{}", branch, branch)
}