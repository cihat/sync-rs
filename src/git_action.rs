use crate::{error::ActionError, project::Project};
use git2::{FetchOptions, PushOptions, Remote, RemoteCallbacks, Repository};
use std::path::Path;

#[derive(Debug)]
#[allow(dead_code)]
pub enum GitActionType {
  PULL,
  PUSH,
}

impl GitActionType {
  pub fn action(
    &self,
    project: &Project,
    remote_path: &str,
    branch: &str,
  ) -> Result<(), ActionError> {
    let repo_path = Path::new(&project.path);
    let repo = Repository::open(repo_path).map_err(ActionError::from)?;
    let mut remote = repo.find_remote(remote_path).map_err(ActionError::from)?;

    match self {
      GitActionType::PULL => {
        self.pull(&repo, &mut remote, branch)?;
      }
      GitActionType::PUSH => {
        self.push(&repo, &mut remote, branch)?;
      }
    }

    Ok(())
  }

  fn create_callbacks() -> RemoteCallbacks<'static> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      git2::Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        std::path::Path::new("/Users/cihatsalik/.ssh/github_rsa/id_rsa"),
        None,
      )
    });

    callbacks.transfer_progress(|stats| {
      println!(
        "Received {}/{} objects in {} bytes",
        stats.received_objects(),
        stats.total_objects(),
        stats.received_bytes()
      );
      true
    });

    callbacks.push_update_reference(|refname, status| {
      if let Some(status) = status {
        println!("Failed to update reference {}: {}", refname, status);
      } else {
        println!("Successfully updated reference {}", refname);
      }
      Ok(())
    });

    callbacks
  }

  //FIX: Fix function to return Result and git2's output to stdout
  fn pull(&self, repo: &Repository, remote: &mut Remote, branch: &str) -> Result<(), ActionError> {
    let callbacks = Self::create_callbacks();

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    remote
      .fetch(&[branch], Some(&mut fetch_opts), None)
      .map_err(ActionError::from)?;
    println!("Pull completed successfully.");

    // Example operation: print the HEAD reference in the repo
    let head = repo.head().map_err(ActionError::from)?;
    println!("Current HEAD: {}", head.name().unwrap_or("unknown"));

    Ok(())
  }

  //FIX: Fix function to return Result and git2's output to stdout
  fn push(&self, repo: &Repository, remote: &mut Remote, branch: &str) -> Result<(), ActionError> {
    let callbacks = Self::create_callbacks();

    let mut push_opts = PushOptions::new();
    push_opts.remote_callbacks(callbacks);

    remote
      .push(
        &[&format!("refs/heads/{}:refs/heads/{}", branch, branch)],
        Some(&mut push_opts),
      )
      .map_err(ActionError::from)?;

    println!("Push completed successfully.");

    // Example operation: print the last commit message in the repo
    let head = repo.head().map_err(ActionError::from)?;
    let commit = head.peel_to_commit().map_err(ActionError::from)?;
    println!(
      "Last commit message: {}",
      commit.message().unwrap_or("No message")
    );

    Ok(())
  }
}
