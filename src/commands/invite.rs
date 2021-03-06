use super::invite_users::*;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Invite users to an organisation by emails
pub enum InviteArgs {
    #[structopt(name = "users")]
    Users(InviteUsersArgs),
}

impl InviteArgs {
    pub fn run(&self) -> Result<()> {
        match self {
            InviteArgs::Users(args) => args.run(),
        }
    }
}
