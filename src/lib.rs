use std::{
    env::{self},
    path::PathBuf,
    process,
};

use structopt::{lazy_static::lazy_static, StructOpt};

pub mod tempalte;

pub static PROJ_NAME: &str = "UserBin";
pub static PROJ_DESC: &str = "A tool to quickly link an executable file to a specified directory.";

#[derive(Debug, StructOpt)]
#[structopt(name = PROJ_NAME, about = PROJ_DESC)]
pub struct CliOpts {
    #[structopt(parse(from_str), short = "")]
    target: PathBuf,

    #[structopt(parse(from_str), long, short = "a")]
    alias: Option<String>,
}

impl CliOpts {
    pub fn target(&self) -> &PathBuf {
        &self.target
    }

    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CLI_ARGS: CliOpts = CliOpts::from_args();

    #[derive(Debug)]
    pub static ref USERBIN_PATH: String = env::var("USERBIN_PATH").unwrap_or_else(|_| {
        println!("未找到环境变量 USERBIN_PATH，需要先设置该环境变量所指向的路径");
        process::exit(1);
    });
}
