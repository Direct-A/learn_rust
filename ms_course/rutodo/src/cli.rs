/*
module parser cli input things
*/
#[cfg(test)]
mod cli_test {
    use super::*;

    #[test]
    fn cli_test() {
        // TODO: 完成测试函数
        CommandlineArgs::from_args();
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(about = "Add task into todo-list")]
    Add {
        text: String,
    },
    #[structopt(about="Remove task(s) in todo-list")]
    Remove {
        positon: usize,
    },
    #[structopt(about="Show task(s) in todo-list")]
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rutodo-cli",
    about = "A cli tool for rutodo.",
    version = "0.0.1"
)]
pub struct CommandlineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(
        parse(from_os_str),
        short,
        long,
        help = "specified a file to store todo-list info"
    )]
    pub file: Option<PathBuf>,
}
