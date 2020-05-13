use super::common;
use super::models::Script;
use crate::filter::Filter;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Apply a script to all local repositories that match a pattern
pub struct ApplyArgs {
    #[structopt(long, short, default_value = "divvun")]
    pub organisation: String,
    #[structopt(long, short)]
    pub regex: Option<Filter>,
    #[structopt(long, short)]
    pub script: Script,
}

impl ApplyArgs {
    pub fn run(&self) -> Result<()> {
        log::trace!("in run");
        let root = common::root()?;
        log::trace!("root");
        let sub_dirs = common::read_dirs_for_org(&self.organisation, &root, self.regex.as_ref())?;
        log::trace!("sub dirs: {:?}", &sub_dirs);

        let script_path = self
            .script
            .path
            .to_str()
            .expect("gut only supports UTF-8 paths now!");
        log::trace!("Script path");

        for dir in sub_dirs {
            log::trace!("dir: {:?}", &dir);
            match common::apply_script(&dir, script_path) {
                Ok(_) => println!(
                    "Applied script {} for dir {:?} successfully",
                    script_path, dir
                ),
                Err(e) => println!(
                    "Failed to apply script {} for dir {:?} because {:?}",
                    script_path, dir, e
                ),
            }
        }

        Ok(())
    }
}
