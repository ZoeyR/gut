use super::create_branch::*;
use super::create_discussion::*;
use super::create_repo::*;
use super::create_team::*;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Create team, discussion, repo to an organisation or create a branch for repositories
pub enum CreateArgs {
    #[structopt(name = "team")]
    Team(CreateTeamArgs),
    #[structopt(name = "discussion")]
    Discussion(CreateDiscussionArgs),
    #[structopt(name = "branch")]
    Branch(CreateBranchArgs),
    #[structopt(name = "repo", aliases = &["repository"])]
    Repo(CreateRepoArgs),
}

impl CreateArgs {
    pub fn run(&self) -> Result<()> {
        match self {
            CreateArgs::Discussion(args) => args.create_discusstion(),
            CreateArgs::Team(args) => args.create_team(),
            CreateArgs::Branch(args) => args.run(),
            CreateArgs::Repo(args) => args.run(),
        }
    }
}
